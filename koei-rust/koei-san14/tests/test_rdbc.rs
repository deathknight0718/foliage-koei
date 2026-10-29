use anyhow::{Context, Result, ensure};
use binrw::{BinRead, BinReaderExt};
use flate2::{Decompress, FlushDecompress, Status};
use std::io::{Cursor, Read, Seek, SeekFrom};
use std::path::Path;

const HEADER_SIZE: u64 = 32;
const ENTRY_HEADER_SIZE: u64 = 56;
const CHUNK_SIZE: usize = 0x4000;

#[derive(BinRead, Debug)]
#[br(little, magic = b"_DRK0000")]
struct RdbHeader {
    size: u32,
    _version: u32,
    entry_count: u32,
    _unknown: u32,
    _data_path: [u8; 8],
}

#[derive(BinRead, Debug)]
#[br(little, magic = b"IDRK0000")]
struct EntryHeader {
    size: u64,
    extra_size: u64,
    unpacked_size: u64,
    _unknown: u32,
    file_id: u32,
    type_id: u32,
    flags: u32,
    _unknown_tail: u64,
}

#[derive(Debug)]
struct Entry {
    header: EntryHeader,
    metadata: Vec<u8>,
    location: String,
}

fn read_index(bytes: &[u8]) -> Result<Vec<Entry>> {
    let mut reader = Cursor::new(bytes);
    let header: RdbHeader = reader.read_le()?;
    ensure!(u64::from(header.size) == HEADER_SIZE, "unexpected RDB header size");
    ensure!(u64::from(header.entry_count) <= (bytes.len() as u64 - HEADER_SIZE) / ENTRY_HEADER_SIZE, "entry count exceeds file length");

    let mut entries = Vec::new();
    for index in 0..header.entry_count {
        let start = reader.position();
        let entry: EntryHeader = reader.read_le().with_context(|| format!("entry {index} header"))?;
        ensure!(entry.size >= ENTRY_HEADER_SIZE, "entry {index} is too short");
        let end = start.checked_add(entry.size).context("entry size overflow")?;
        ensure!(end <= bytes.len() as u64, "entry {index} extends past end of file");
        let extra_len = usize::try_from(entry.extra_size).context("location length overflow")?;
        let variable_start = usize::try_from(start + ENTRY_HEADER_SIZE)?;
        let variable_end = usize::try_from(end)?;
        let variable = &bytes[variable_start..variable_end];
        ensure!(extra_len <= variable.len(), "entry {index} location is too long");
        let (metadata, location) = variable.split_at(variable.len() - extra_len);
        let location = if location.is_empty() {
            String::new()
        } else {
            let location = location.strip_suffix(&[0]).context("location is not null-terminated")?;
            ensure!(!location.contains(&0), "entry {index} location contains an embedded null");
            std::str::from_utf8(location)?.to_owned()
        };
        let aligned_end = end.checked_add(3).context("entry alignment overflow")? & !3;
        ensure!(aligned_end <= bytes.len() as u64, "entry {index} padding exceeds file length");
        ensure!(bytes[variable_end..usize::try_from(aligned_end)?].iter().all(|&byte| byte == 0), "entry {index} has nonzero padding");
        reader.set_position(aligned_end);
        entries.push(Entry { header: entry, metadata: metadata.to_vec(), location });
    }
    ensure!(reader.position() == bytes.len() as u64, "unexpected trailing index data");
    Ok(entries)
}

fn decode_block(bytes: &[u8]) -> Result<(EntryHeader, Vec<u8>)> {
    let mut reader = Cursor::new(bytes);
    let header: EntryHeader = reader.read_le()?;
    ensure!(header.size == bytes.len() as u64, "incorrect block size");
    ensure!(header.extra_size == header.size - ENTRY_HEADER_SIZE, "incorrect block payload size");
    let expected_len = usize::try_from(header.unpacked_size)?;
    let mut output = Vec::new();
    loop {
        let chunk_len: u32 = reader.read_le().context("missing chunk length or terminator")?;
        if chunk_len == 0 {
            break;
        }
        let chunk_len = usize::try_from(chunk_len)?;
        let start = usize::try_from(reader.position())?;
        let end = start.checked_add(chunk_len).context("chunk length overflow")?;
        let chunk = bytes.get(start..end).context("compressed chunk exceeds block")?;
        let mut inflater = Decompress::new(true);
        let mut decoded = [0; CHUNK_SIZE];
        let status = inflater.decompress(chunk, &mut decoded, FlushDecompress::Finish)?;
        ensure!(status == Status::StreamEnd, "incomplete zlib stream");
        ensure!(inflater.total_in() == chunk_len as u64, "unused bytes in zlib chunk");
        let decoded_len = usize::try_from(inflater.total_out())?;
        ensure!(output.len().checked_add(decoded_len).is_some_and(|len| len <= expected_len), "decoded data exceeds expected size");
        output.extend_from_slice(&decoded[..decoded_len]);
        reader.set_position(u64::try_from(end)?);
    }
    ensure!(reader.position() == bytes.len() as u64, "unexpected data after chunk terminator");
    ensure!(output.len() == expected_len, "decoded size does not match block header");
    Ok((header, output))
}

fn index_path() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..").join(".data").join("system.rdb")
}

#[test]
fn decode_system_rdb_index() -> Result<()> {
    let mut bytes = std::fs::read(index_path())?;
    let entries = read_index(&bytes)?;
    assert_eq!(entries.len(), 22_792);
    let first = &entries[0];
    assert_eq!(first.header.file_id, 0x0005_cd86);
    assert_eq!(first.header.type_id, 0x7bcd_279f);
    assert_eq!(first.header.flags, 0x0012_0000);
    assert_eq!(first.header.unpacked_size, 30_096);
    assert!(first.metadata.is_empty());
    assert_eq!(first.location, "15becc7c@1c9c#e");
    assert_eq!(entries[177].header.flags, 0x0011_0000);
    assert!(entries[177].location.is_empty());
    assert_eq!(entries[220].metadata.len(), 13);

    bytes[HEADER_SIZE as usize + 8..HEADER_SIZE as usize + 16].copy_from_slice(&u64::MAX.to_le_bytes());
    assert!(read_index(&bytes).is_err());
    Ok(())
}

#[test]
fn decode_system_rdb_first_asset() -> Result<()> {
    let entries = read_index(&std::fs::read(index_path())?)?;
    let first = &entries[0];
    let (offset, size) = first.location.strip_suffix("#e").context("unexpected archive suffix")?.split_once('@').context("invalid location")?;
    let offset = u64::from_str_radix(offset, 16)?;
    let size = usize::from_str_radix(size, 16)?;
    let archive_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..").join(".resources").join("Romance_of_the_Three_Kingdoms_14").join("0002").join("system.rdb.bin14");
    let mut archive = std::fs::File::open(archive_path)?;
    archive.seek(SeekFrom::Start(offset))?;
    let mut block = vec![0; size];
    archive.read_exact(&mut block)?;
    let (header, decoded) = decode_block(&block)?;
    assert_eq!(header.file_id, first.header.file_id);
    assert_eq!(header.type_id, first.header.type_id);
    assert_eq!(header.unpacked_size, first.header.unpacked_size);
    assert_eq!(decoded.len(), 30_096);
    assert_eq!(&decoded[..8], b"_S2G3000");

    assert!(decode_block(&block[..block.len() - 1]).is_err());
    let mut invalid = block.clone();
    invalid[ENTRY_HEADER_SIZE as usize..ENTRY_HEADER_SIZE as usize + 4].copy_from_slice(&u32::MAX.to_le_bytes());
    assert!(decode_block(&invalid).is_err());
    invalid = block;
    invalid[ENTRY_HEADER_SIZE as usize + 4] ^= 0xff;
    assert!(decode_block(&invalid).is_err());
    Ok(())
}
