use anyhow::Result as AnyhowResult;
use csv::Writer;
use koei_san14::domain::{area, city, pass, province};
use koei_san14::fixc::Fixdata;
use koei_san14::lwcc;
use serde::Serialize;
use std::path::Path;

const FIXDATAEXCE_PATH: &str = "../../.resources/Romance_of_the_Three_Kingdoms_14/0010_SC/fixdataexce.s14";
const FIXDATA_PATH: &str = "../../.resources/Romance_of_the_Three_Kingdoms_14/0010_SC/fixdata.s14";
const DATA_DIRECTORY: &str = "../../.data";
const FIXDATAEXCE_LWC_OFFSET: usize = 0x2F2;
const FIXDATA_LWC_OFFSET: usize = 0x1E0;

fn write_to_csv<D: Serialize>(records: &[D]) -> AnyhowResult<String> {
    let mut writer = Writer::from_writer(Vec::new());
    records.iter().try_for_each(|record| writer.serialize(record))?;
    writer.flush()?;
    Ok(String::from_utf8(writer.into_inner()?)?)
}

#[test]
fn test_tables() {
    let bytes = std::fs::read(FIXDATAEXCE_PATH).unwrap_or_else(|_| panic!("read {}", FIXDATAEXCE_PATH));
    let fixdata = Fixdata::try_from(bytes.as_slice()).expect("parse fixdataexce.s14");
    for table in &fixdata.tables {
        match table.index {
            province::INDEX => {
                let records = table.records::<province::RecordProvince>();
                insta::assert_snapshot!("province", write_to_csv(&records).expect("write province CSV"));
            }
            city::INDEX => {
                let records = table.records::<city::RecordCity>();
                insta::assert_snapshot!("city", write_to_csv(&records).expect("write city CSV"));
            }
            pass::INDEX => {
                let records = table.records::<pass::RecordPass>();
                insta::assert_snapshot!("pass", write_to_csv(&records).expect("write pass CSV"));
            }
            area::INDEX => {
                let records = table.records::<area::RecordArea>();
                insta::assert_snapshot!("area", write_to_csv(&records).expect("write area CSV"));
            }
            _ => println!("table 0x{:02X}: {} bytes", table.index, table.bytes.len()),
        }
    }
}

#[test]
fn test_data() -> AnyhowResult<()> {
    let bytes = std::fs::read(FIXDATAEXCE_PATH)?;
    let fixdata = Fixdata::try_from(bytes.as_slice())?;
    let directory = Path::new(DATA_DIRECTORY);
    std::fs::create_dir_all(directory)?;
    for table in fixdata.tables {
        std::fs::write(directory.join(format!("table{}.bin", table.index)), table.bytes)?;
    }
    Ok(())
}

#[test]
fn test_decompress_data() -> AnyhowResult<()> {
    let bytes = std::fs::read(FIXDATAEXCE_PATH)?;
    let decompressed = lwcc::decompress(&bytes, FIXDATAEXCE_LWC_OFFSET)?;
    let directory = Path::new(DATA_DIRECTORY);
    std::fs::create_dir_all(directory)?;
    std::fs::write(directory.join("fixdataexce.bin"), decompressed)?;
    Ok(())
}

#[test]
fn test_decompress_fixdata() -> AnyhowResult<()> {
    let bytes = std::fs::read(FIXDATA_PATH)?;
    let decompressed = lwcc::decompress(&bytes, FIXDATA_LWC_OFFSET)?;
    let directory = Path::new(DATA_DIRECTORY);
    std::fs::create_dir_all(directory)?;
    std::fs::write(directory.join("fixdata.bin"), decompressed)?;
    Ok(())
}
