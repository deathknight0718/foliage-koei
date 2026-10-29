mod commons;

use koei_san14::rdbc::{self, Archive, AssetBlock, CHUNK_SIZE, ENTRY_HEADER_SIZE, EntryHeader, Location};
use std::io::Cursor;

fn fixture(data: Vec<u8>) -> AssetBlock {
    AssetBlock { header: EntryHeader { size: 0, extra_size: 0, unpacked_size: 0, unknown: 42, file_id: 123, type_id: 456, flags: 0x100000, unknown_tail: 789 }, metadata: vec![1, 2, 3, 4, 5], data }
}

#[test]
fn block_roundtrip_and_invalid_data() -> anyhow::Result<()> {
    for len in [0, 1, CHUNK_SIZE, CHUNK_SIZE + 1, CHUNK_SIZE * 3] {
        let block = fixture((0..len).map(|i| (i % 251) as u8).collect());
        let encoded = rdbc::encode_block(&block)?;
        let decoded = rdbc::decode_block(&encoded)?;
        assert_eq!(decoded.data, block.data);
        assert_eq!(decoded.metadata, block.metadata);
        assert_eq!(decoded.header.file_id, block.header.file_id);
        assert_eq!(decoded.header.unknown_tail, block.header.unknown_tail);
        assert_eq!(decoded.header.size, encoded.len() as u64);
        assert_eq!(decoded.header.unpacked_size, len as u64);
        assert!(rdbc::decode_block(&encoded[..encoded.len() - 1]).is_err());
    }
    let mut encoded = rdbc::encode_block(&fixture(vec![1; 100]))?;
    let payload_start = ENTRY_HEADER_SIZE as usize + 5;
    encoded[payload_start..payload_start + 4].copy_from_slice(&u32::MAX.to_le_bytes());
    assert!(rdbc::decode_block(&encoded).is_err());
    let mut encoded = rdbc::encode_block(&fixture(vec![1; 100]))?;
    encoded[payload_start + 4] ^= 0xff;
    assert!(rdbc::decode_block(&encoded).is_err());
    Ok(())
}

#[test]
fn locations() -> anyhow::Result<()> {
    assert_eq!(rdbc::parse_location("")?, None);
    assert_eq!(rdbc::parse_location("15becc7c@1c9c#e")?, Some(Location { archive: Archive::Bin14, offset: 0x15becc7c, size: 0x1c9c }));
    assert_eq!(rdbc::parse_location("15498bab@11bb6")?.unwrap().archive, Archive::Bin);
    for invalid in ["#e", "0@1#f", "0@1#e#e", "-1@0", "0@", "0@1@2"] {
        assert!(rdbc::parse_location(invalid).is_err(), "{invalid}");
    }
    Ok(())
}

#[test]
fn decode_system_rdb_index() -> anyhow::Result<()> {
    let mut bytes = bytes!("system", ".rdb")?;
    let index = rdbc::decode_index(&bytes)?;
    assert_eq!(index.entries.len(), 22_792);
    let first = &index.entries[0];
    assert_eq!(first.header.file_id, 0x0005_cd86);
    assert_eq!(first.header.type_id, 0x7bcd_279f);
    assert_eq!(first.header.flags, 0x0012_0000);
    assert_eq!(first.header.unpacked_size, 30_096);
    assert!(first.metadata.is_empty());
    assert_eq!(first.location, "15becc7c@1c9c#e");
    assert_eq!(index.entries[177].header.flags, 0x0011_0000);
    assert!(index.entries[177].location.is_empty());
    assert_eq!(index.entries[220].metadata.len(), 13);
    assert_eq!(rdbc::encode_index(&index)?, bytes);
    let mut changed = index.clone();
    changed.entries[0].location.push('x');
    assert!(rdbc::encode_index(&changed).is_err());
    bytes[HEADER_SIZE..HEADER_SIZE + 8].copy_from_slice(&u64::MAX.to_le_bytes());
    assert!(rdbc::decode_index(&bytes).is_err());
    Ok(())
}

const HEADER_SIZE: usize = 32 + 8;

#[test]
fn decode_system_rdb_assets() -> anyhow::Result<()> {
    let index = rdbc::decode_index(&bytes!("system", ".rdb")?)?;
    let mut archive = std::fs::File::open(path!("system", ".rdb.bin14"))?;
    let first = rdbc::read_asset(&index.entries[0], &mut archive)?;
    assert_eq!(first.data.len(), 30_096);
    assert_eq!(&first.data[..8], b"_S2G3000");
    let encoded = rdbc::encode_block(&first)?;
    assert_eq!(rdbc::decode_block(&encoded)?.data, first.data);
    let with_metadata = rdbc::read_asset(&index.entries[711], &mut archive)?;
    assert_eq!(with_metadata.metadata.len(), 13);
    let encoded = rdbc::encode_block(&with_metadata)?;
    let decoded = rdbc::decode_block(&encoded)?;
    assert_eq!(decoded.metadata, with_metadata.metadata);
    assert_eq!(decoded.data, with_metadata.data);
    assert!(rdbc::read_asset(&index.entries[0], &mut Cursor::new(vec![0; 8])).is_err());
    assert!(rdbc::read_asset(&index.entries[177], &mut archive).is_err());
    Ok(())
}
