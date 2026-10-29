//! SAN14 RDB index and IDRK asset block codec.
//!
//! Index locations without a suffix refer to `system.rdb.bin`; `#e` refers to
//! `system.rdb.bin14` for the supplied SAN14 data. Entries without locations
//! cannot be resolved from these archives. The original filenames are not
//! stored in the index.
//!
//! Index encoding preserves each entry's original size and its opaque metadata.
//! Block encoding preserves the opaque prefix and header fields, but recompresses
//! the payload; callers must update the index location when relocating a block.

use anyhow::{Context, Result, ensure};
use binrw::{BinRead, BinReaderExt, BinWrite, BinWriterExt};
use flate2::{Compression, Decompress, FlushDecompress, Status, write::ZlibEncoder};
use std::io::{Cursor, Read, Seek, SeekFrom, Write};

pub const HEADER_SIZE: u64 = 32;
pub const ENTRY_HEADER_SIZE: u64 = 56;
pub const CHUNK_SIZE: usize = 0x4000;

#[derive(BinRead, BinWrite, Debug, Clone, PartialEq, Eq)]
#[brw(little, magic = b"_DRK0000")]
pub struct RdbHeader {
    pub size: u32,
    pub version: u32,
    pub entry_count: u32,
    pub unknown: u32,
    pub data_path: [u8; 8],
}

#[derive(BinRead, BinWrite, Debug, Clone, PartialEq, Eq)]
#[brw(little, magic = b"IDRK0000")]
pub struct EntryHeader {
    pub size: u64,
    pub extra_size: u64,
    pub unpacked_size: u64,
    pub unknown: u32,
    pub file_id: u32,
    pub type_id: u32,
    pub flags: u32,
    pub unknown_tail: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub header: EntryHeader,
    pub metadata: Vec<u8>,
    pub location: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RdbIndex {
    pub header: RdbHeader,
    pub entries: Vec<Entry>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssetBlock {
    pub header: EntryHeader,
    pub metadata: Vec<u8>,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Archive {
    Bin,
    Bin14,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Location {
    pub archive: Archive,
    pub offset: u64,
    pub size: u64,
}

/// Parse the SAN14 textual archive location; an empty location is unresolved.
pub fn parse_location(location: &str) -> Result<Option<Location>> {
    if location.is_empty() {
        return Ok(None);
    }
    let (address, archive) = match location.split_once('#') {
        Some((address, "e")) => (address, Archive::Bin14),
        Some((_, suffix)) => anyhow::bail!("unsupported RDB archive suffix #{suffix}"),
        None => (location, Archive::Bin),
    };
    let (offset, size) = address.split_once('@').context("RDB location is missing '@'")?;
    ensure!(!offset.is_empty() && !size.is_empty() && offset.bytes().all(|b| b.is_ascii_hexdigit()) && size.bytes().all(|b| b.is_ascii_hexdigit()), "invalid RDB location {location:?}");
    Ok(Some(Location { archive, offset: u64::from_str_radix(offset, 16).context("invalid RDB location offset")?, size: u64::from_str_radix(size, 16).context("invalid RDB location size")? }))
}

pub fn decode_index(bytes: &[u8]) -> Result<RdbIndex> {
    let mut reader = Cursor::new(bytes);
    let header: RdbHeader = reader.read_le().context("RDB header")?;
    ensure!(u64::from(header.size) == HEADER_SIZE, "unexpected RDB header size {}", header.size);
    ensure!(u64::from(header.entry_count) <= (bytes.len() as u64 - HEADER_SIZE) / ENTRY_HEADER_SIZE, "RDB entry count exceeds file length");

    let mut entries = Vec::new();
    for index in 0..header.entry_count {
        let start = reader.position();
        let entry: EntryHeader = reader.read_le().with_context(|| format!("RDB entry {index} header"))?;
        ensure!(entry.size >= ENTRY_HEADER_SIZE, "RDB entry {index} is too short");
        let end = start.checked_add(entry.size).context("RDB entry size overflow")?;
        ensure!(end <= bytes.len() as u64, "RDB entry {index} extends past end of file");
        let variable = &bytes[usize::try_from(start + ENTRY_HEADER_SIZE)?..usize::try_from(end)?];
        let extra_len = usize::try_from(entry.extra_size).context("RDB location length overflow")?;
        ensure!(extra_len <= variable.len(), "RDB entry {index} location is too long");
        let (metadata, location) = variable.split_at(variable.len() - extra_len);
        let location = if location.is_empty() {
            String::new()
        } else {
            let location = location.strip_suffix(&[0]).context("RDB location is not null-terminated")?;
            ensure!(!location.contains(&0), "RDB entry {index} location contains an embedded null");
            std::str::from_utf8(location).context("RDB location is not UTF-8")?.to_owned()
        };
        let aligned_end = end.checked_add(3).context("RDB entry alignment overflow")? & !3;
        ensure!(aligned_end <= bytes.len() as u64, "RDB entry {index} padding exceeds file length");
        ensure!(bytes[usize::try_from(end)?..usize::try_from(aligned_end)?].iter().all(|&byte| byte == 0), "RDB entry {index} has nonzero padding");
        reader.set_position(aligned_end);
        entries.push(Entry { header: entry, metadata: metadata.to_vec(), location });
    }
    ensure!(reader.position() == bytes.len() as u64, "unexpected trailing RDB index data");
    Ok(RdbIndex { header, entries })
}

/// Encode an index without changing the size of any existing entry.
pub fn encode_index(index: &RdbIndex) -> Result<Vec<u8>> {
    ensure!(u64::from(index.header.size) == HEADER_SIZE, "unexpected RDB header size");
    let mut header = index.header.clone();
    header.entry_count = u32::try_from(index.entries.len()).context("too many RDB entries")?;
    let mut writer = Cursor::new(Vec::new());
    writer.write_le(&header)?;
    for (number, entry) in index.entries.iter().enumerate() {
        ensure!(!entry.location.as_bytes().contains(&0), "RDB entry {number} location contains a null");
        let location_len = if entry.location.is_empty() && entry.header.extra_size == 0 { 0 } else { entry.location.len().checked_add(1).context("RDB location length overflow")? };
        let length = usize::try_from(ENTRY_HEADER_SIZE)?.checked_add(entry.metadata.len()).and_then(|n| n.checked_add(location_len)).context("RDB entry size overflow")?;
        ensure!(u64::try_from(length)? == entry.header.size, "RDB entry {number} size changed; preserve its original size");
        ensure!(u64::try_from(location_len)? == entry.header.extra_size, "RDB entry {number} location length differs from its header");
        writer.write_le(&entry.header)?;
        writer.write_all(&entry.metadata)?;
        if location_len != 0 {
            writer.write_all(entry.location.as_bytes())?;
            writer.write_all(&[0])?;
        }
        let aligned = writer.position().checked_add(3).context("RDB entry alignment overflow")? & !3;
        while writer.position() < aligned {
            writer.write_all(&[0])?;
        }
    }
    Ok(writer.into_inner())
}

/// Decode an IDRK block, locating the compressed payload from the end of the block.
pub fn decode_block(bytes: &[u8]) -> Result<AssetBlock> {
    let mut reader = Cursor::new(bytes);
    let header: EntryHeader = reader.read_le().context("IDRK block header")?;
    ensure!(header.size == bytes.len() as u64, "incorrect IDRK block size");
    ensure!(header.extra_size <= header.size - ENTRY_HEADER_SIZE, "IDRK payload exceeds block size");
    let payload_start = usize::try_from(header.size - header.extra_size)?;
    let metadata = bytes[usize::try_from(ENTRY_HEADER_SIZE)?..payload_start].to_vec();
    reader.set_position(u64::try_from(payload_start)?);
    let expected_len = usize::try_from(header.unpacked_size).context("IDRK unpacked size exceeds addressable memory")?;
    let mut output = Vec::new();
    loop {
        let chunk_len: u32 = reader.read_le().context("missing IDRK chunk length or terminator")?;
        if chunk_len == 0 {
            break;
        }
        let start = usize::try_from(reader.position())?;
        let end = start.checked_add(usize::try_from(chunk_len)?).context("IDRK chunk length overflow")?;
        let chunk = bytes.get(start..end).context("compressed IDRK chunk exceeds block")?;
        let mut inflater = Decompress::new(true);
        let mut decoded = [0; CHUNK_SIZE];
        let status = inflater.decompress(chunk, &mut decoded, FlushDecompress::Finish).context("invalid IDRK zlib stream")?;
        ensure!(status == Status::StreamEnd, "incomplete IDRK zlib stream or chunk exceeds 0x{CHUNK_SIZE:X} bytes");
        ensure!(inflater.total_in() == u64::from(chunk_len), "unused bytes in IDRK zlib chunk");
        let decoded_len = usize::try_from(inflater.total_out())?;
        ensure!(output.len().checked_add(decoded_len).is_some_and(|len| len <= expected_len), "decoded IDRK data exceeds expected size");
        output.extend_from_slice(&decoded[..decoded_len]);
        reader.set_position(u64::try_from(end)?);
    }
    ensure!(reader.position() == bytes.len() as u64, "unexpected data after IDRK chunk terminator");
    ensure!(output.len() == expected_len, "decoded IDRK size does not match block header");
    Ok(AssetBlock { header, metadata, data: output })
}

/// Recompress an asset using independent 0x4000-byte zlib chunks.
pub fn encode_block(block: &AssetBlock) -> Result<Vec<u8>> {
    let mut payload = Vec::new();
    for chunk in block.data.chunks(CHUNK_SIZE) {
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(chunk).context("compressing IDRK chunk")?;
        let compressed = encoder.finish().context("finishing IDRK chunk")?;
        payload.extend_from_slice(&u32::try_from(compressed.len()).context("compressed IDRK chunk is too large")?.to_le_bytes());
        payload.extend_from_slice(&compressed);
    }
    payload.extend_from_slice(&0u32.to_le_bytes());
    let mut header = block.header.clone();
    header.extra_size = u64::try_from(payload.len())?;
    header.size = ENTRY_HEADER_SIZE.checked_add(u64::try_from(block.metadata.len())?).and_then(|n| n.checked_add(header.extra_size)).context("IDRK block size overflow")?;
    header.unpacked_size = u64::try_from(block.data.len())?;
    let mut writer = Cursor::new(Vec::new());
    writer.write_le(&header)?;
    writer.write_all(&block.metadata)?;
    writer.write_all(&payload)?;
    Ok(writer.into_inner())
}

/// Read and validate an asset from the archive selected by its index location.
pub fn read_asset<R: Read + Seek>(entry: &Entry, archive: &mut R) -> Result<AssetBlock> {
    let location = parse_location(&entry.location)?.context("RDB entry has no archive location")?;
    let archive_size = archive.seek(SeekFrom::End(0)).context("querying archive length")?;
    let end = location.offset.checked_add(location.size).context("RDB location range overflow")?;
    ensure!(end <= archive_size, "RDB location exceeds archive length");
    let size = usize::try_from(location.size).context("RDB block size exceeds addressable memory")?;
    archive.seek(SeekFrom::Start(location.offset)).context("seeking to RDB block")?;
    let mut bytes = vec![0; size];
    archive.read_exact(&mut bytes).context("reading RDB block")?;
    let block = decode_block(&bytes)?;
    ensure!(block.header.file_id == entry.header.file_id && block.header.type_id == entry.header.type_id && block.header.unpacked_size == entry.header.unpacked_size, "IDRK block does not match its RDB entry");
    Ok(block)
}
