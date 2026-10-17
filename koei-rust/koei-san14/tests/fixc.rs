use koei_san14::domain::area;
use koei_san14::domain::area::RecordArea;
use koei_san14::domain::city;
use koei_san14::domain::city::RecordCity;
use koei_san14::domain::region;
use koei_san14::domain::region::RecordRegion;
use koei_san14::fixc::{DecodedFixdata, EXPECTED_HEADER_VALUE, Fixdata};
use koei_san14::lwcc::{LWC_MAGIC, SYMBOL_BASE, decompress_lwc};

const LWC_HEADER_SIZE: usize = 0x10C;

fn append_symbol(bits: &mut Vec<bool>, symbol: usize) {
    let (length, value) = (1..SYMBOL_BASE.len())
        .find_map(|length| {
            let value = symbol.checked_sub(SYMBOL_BASE[length])?;
            (value < (1 << length)).then_some((length, value))
        })
        .expect("test symbol must be encodable");

    for shift in (0..length).rev() {
        bits.push(shift != 0);
    }
    for shift in (0..length).rev() {
        bits.push(value & (1 << shift) != 0);
    }
}

fn literal_lwc(bytes: &[u8]) -> Vec<u8> {
    let mut encoded = vec![0; LWC_HEADER_SIZE];
    encoded[..4].copy_from_slice(&LWC_MAGIC.to_le_bytes());
    encoded[4..8].copy_from_slice(&(bytes.len() as u32).to_le_bytes());
    for (index, entry) in encoded[12..12 + 0x0100].iter_mut().enumerate() {
        *entry = index as u8;
    }
    let mut bits = Vec::new();
    for byte in bytes {
        append_symbol(&mut bits, usize::from(*byte));
    }
    for chunk in bits.chunks(8) {
        encoded.push(chunk.iter().enumerate().fold(0, |value, (index, bit)| value | (u8::from(*bit) << (7 - index))));
    }
    encoded
}

fn fixdata_fixture() -> Vec<u8> {
    let mut bytes = Vec::from(EXPECTED_HEADER_VALUE.to_le_bytes());
    bytes.extend_from_slice(area::FLAG);
    bytes.extend_from_slice(&[0xAA, 0xBB]);
    bytes
}

fn append_table(bytes: &mut Vec<u8>, identifier: u32, count: u32, record_size: usize) -> usize {
    bytes.extend_from_slice(&identifier.to_le_bytes());
    bytes.extend_from_slice(&count.to_le_bytes());
    let records_offset = bytes.len();
    bytes.resize(records_offset + count as usize * record_size, 0);
    records_offset
}

fn sc_prefix_fixture() -> Vec<u8> {
    let mut bytes = Vec::from(EXPECTED_HEADER_VALUE.to_le_bytes());
    bytes.extend_from_slice(&0x0C_u32.to_le_bytes());
    bytes.resize(bytes.len() + 0x0C * 0x14, 0);
    let area_offset = append_table(&mut bytes, 0, 0x17, 0x20);
    bytes[area_offset..area_offset + 10].copy_from_slice(&[0x7D, 0x5E, 0xDE, 0x5D, 0, 0, 0, 0, 0, 0]);
    bytes[area_offset + 0x1E] = 1;
    bytes[area_offset + 0x1F] = 1;
    let city_offset = append_table(&mut bytes, 1, 0x34, 0x35);
    bytes[city_offset..city_offset + 8].copy_from_slice(&[0x44, 0x89, 0x73, 0x5E, 0, 0, 0, 0]);
    bytes[city_offset + 0x27] = 1;
    bytes[city_offset + 0x34] = 1;
    let region_offset = append_table(&mut bytes, 2, 0x0B, 0x28);
    bytes[region_offset..region_offset + 8].copy_from_slice(&[0xF6, 0x58, 0x73, 0x51, 0, 0, 0, 0]);
    bytes[region_offset + 0x27] = 1;
    bytes.extend_from_slice(&3_u32.to_le_bytes());
    bytes
}

#[test]
fn records_match_confirmed_layouts() {
    assert_eq!(std::mem::size_of::<RecordArea>(), 0x20);
    assert_eq!(std::mem::size_of::<RecordCity>(), 0x35);
    assert_eq!(std::mem::size_of::<RecordRegion>(), 0x28);
    assert_eq!(area::FLAG, b"SN14SCEXVER0001\0");
    assert_eq!(area::INDEX, 0x00);
    assert_eq!(city::INDEX, 0x01);
    assert_eq!(region::INDEX, 0x02);
}

#[test]
fn parses_confirmed_header() {
    let bytes = fixdata_fixture();
    let document = Fixdata::parse::<RecordArea>(&bytes).unwrap();
    assert_eq!(document.val, EXPECTED_HEADER_VALUE);
    assert_eq!(document.payload, [0xAA, 0xBB]);
}

#[test]
fn rejects_fixdata_header() {
    let mut bytes = fixdata_fixture();
    bytes[..4].copy_from_slice(&0x33_u32.to_le_bytes());
    assert_eq!(Fixdata::parse::<RecordArea>(&bytes).unwrap_err().to_string(), "expected fixdataexce header value 0x66, found 0x33");
}

#[test]
fn decompresses_lwc_literals() {
    assert_eq!(decompress_lwc(&literal_lwc(b"SAN14")).unwrap(), b"SAN14");
}

#[test]
fn rejects_non_lwc_payload() {
    assert_eq!(decompress_lwc(&[0; LWC_HEADER_SIZE]).unwrap_err().to_string(), "expected LWC magic 0x1A43574C, found 0x0");
}

#[test]
fn reads_each_domain_table_independently() {
    let payload = sc_prefix_fixture();
    let data = DecodedFixdata { val: EXPECTED_HEADER_VALUE, flag: *area::FLAG, payload };
    let area = data.read::<RecordArea>().unwrap();
    assert_eq!(area.len(), 0x17);
    assert_eq!(area[0].lp00, [0x7D, 0x5E, 0xDE, 0x5D, 0, 0, 0, 0, 0, 0]);
    assert_eq!(area[0].lp1e, 1);
    assert_eq!(area[0].lp1f, 1);
    let city = data.read::<RecordCity>().unwrap();
    assert_eq!(city.len(), 0x34);
    assert_eq!(city[0].lp00, [0x44, 0x89, 0x73, 0x5E, 0, 0, 0, 0]);
    assert_eq!(city[0].lp27, 1);
    assert_eq!(city[0].lp34, 1);
    let region = data.read::<RecordRegion>().unwrap();
    assert_eq!(region.len(), 0x0B);
    assert_eq!(region[0].lp00, [0xF6, 0x58, 0x73, 0x51, 0, 0, 0, 0]);
    assert_eq!(region[0].lp27, 1);
}
