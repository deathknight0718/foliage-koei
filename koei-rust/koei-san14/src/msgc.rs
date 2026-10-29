use anyhow::{Context, Result, ensure};
use flate2::{Compression, read::ZlibDecoder, write::ZlibEncoder};
use std::io::{Read, Write};

const MAGIC: [u8; 4] = *b"zp1\0";
const HEADER_SIZE: usize = 0x800;
const BLOCK_SIZE: usize = 0x40000;
const BLOCK_ALIGNMENT: usize = 0x80;
const BLOCK_LENGTH_OFFSET: usize = 0x10;

pub fn decode(bytes: &[u8]) -> Result<Vec<u8>> {
    ensure!(bytes.len() >= HEADER_SIZE, "ZP1 file requires a 0x{HEADER_SIZE:X}-byte header, found 0x{:X}", bytes.len());
    ensure!(bytes[..4] == MAGIC, "ZP1 file has an invalid magic number");

    let uncompressed_size = usize::try_from(read_u32(bytes, 0x04)?).context("ZP1 uncompressed size exceeds addressable memory")?;
    let block_size = usize::try_from(read_u32(bytes, 0x08)?).context("ZP1 block size exceeds addressable memory")?;
    ensure!(block_size == BLOCK_SIZE, "ZP1 block size must be 0x{BLOCK_SIZE:X}, found 0x{block_size:X}");

    let block_count = usize::try_from(read_u32(bytes, 0x0C)?).context("ZP1 block count exceeds addressable memory")?;
    let expected_block_count = uncompressed_size.div_ceil(BLOCK_SIZE);
    ensure!(block_count == expected_block_count, "ZP1 block count {block_count} does not match uncompressed size 0x{uncompressed_size:X}");

    let length_table_end = BLOCK_LENGTH_OFFSET.checked_add(block_count.checked_mul(4).context("ZP1 block-length table size overflow")?).context("ZP1 block-length table end overflow")?;
    ensure!(length_table_end <= HEADER_SIZE, "ZP1 block-length table exceeds the header");

    let mut output = Vec::with_capacity(uncompressed_size);
    let mut offset = HEADER_SIZE;

    for index in 0..block_count {
        let stored_length = usize::try_from(read_u32(bytes, BLOCK_LENGTH_OFFSET + index * 4)?).context("ZP1 stored block length exceeds addressable memory")?;
        ensure!(stored_length >= 4, "ZP1 block {index} is shorter than its length prefix");
        let block_end = offset.checked_add(stored_length).context("ZP1 block end overflow")?;
        ensure!(block_end <= bytes.len(), "ZP1 block {index} exceeds the file size");

        let compressed_length = usize::try_from(read_u32(bytes, offset)?).context("ZP1 compressed block length exceeds addressable memory")?;
        ensure!(compressed_length + 4 == stored_length, "ZP1 block {index} length prefix does not match the header");

        let mut decoder = ZlibDecoder::new(&bytes[offset + 4..block_end]);
        let mut block = Vec::new();
        decoder.read_to_end(&mut block).with_context(|| format!("failed to decompress ZP1 block {index}"))?;

        let expected_length = (uncompressed_size - output.len()).min(BLOCK_SIZE);
        ensure!(block.len() == expected_length, "ZP1 block {index} decompressed to 0x{:X} bytes, expected 0x{expected_length:X}", block.len());
        output.extend_from_slice(&block);

        offset = align_block_end(block_end)?;
    }

    ensure!(output.len() == uncompressed_size, "ZP1 decompressed size 0x{:X} does not match header size 0x{uncompressed_size:X}", output.len());
    Ok(output)
}

pub fn encode(data: &[u8]) -> Result<Vec<u8>> {
    let uncompressed_size = u32::try_from(data.len()).context("ZP1 payload exceeds the maximum encodable size")?;
    let block_count = data.len().div_ceil(BLOCK_SIZE);
    ensure!(BLOCK_LENGTH_OFFSET + block_count * 4 <= HEADER_SIZE, "ZP1 payload requires too many blocks for its header");

    let mut blocks = Vec::with_capacity(block_count);
    for chunk in data.chunks(BLOCK_SIZE) {
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(chunk).context("failed to compress ZP1 block")?;
        let compressed = encoder.finish().context("failed to finalize ZP1 block compression")?;
        let stored_length = u32::try_from(compressed.len() + 4).context("ZP1 compressed block exceeds the maximum encodable size")?;
        blocks.push((stored_length, compressed));
    }

    let mut encoded = vec![0; HEADER_SIZE];
    encoded[..4].copy_from_slice(&MAGIC);
    encoded[0x04..0x08].copy_from_slice(&uncompressed_size.to_le_bytes());
    encoded[0x08..0x0C].copy_from_slice(&(BLOCK_SIZE as u32).to_le_bytes());
    encoded[0x0C..0x10].copy_from_slice(&(block_count as u32).to_le_bytes());

    for (index, (stored_length, compressed)) in blocks.iter().enumerate() {
        encoded[BLOCK_LENGTH_OFFSET + index * 4..BLOCK_LENGTH_OFFSET + index * 4 + 4].copy_from_slice(&stored_length.to_le_bytes());
        encoded.extend_from_slice(&(compressed.len() as u32).to_le_bytes());
        encoded.extend_from_slice(compressed);
        encoded.resize(align_block_end(encoded.len())?, 0);
    }

    Ok(encoded)
}

fn read_u32(bytes: &[u8], offset: usize) -> Result<u32> {
    let value = bytes.get(offset..offset + 4).context("ZP1 header is truncated")?;
    Ok(u32::from_le_bytes(value.try_into().expect("u32 slice length")))
}

fn align_block_end(offset: usize) -> Result<usize> {
    offset.checked_add(BLOCK_ALIGNMENT - 1).map(|value| value & !(BLOCK_ALIGNMENT - 1)).context("ZP1 block alignment overflow")
}
