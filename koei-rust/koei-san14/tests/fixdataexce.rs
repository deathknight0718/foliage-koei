use anyhow::Result as AnyhowResult;
use csv::Writer;
use koei_san14::domain::{area, city, pass, table4, table5, table6, table7, table8, table9, table10, table11, table12, table13};
use koei_san14::fixc::Fixdata;
use serde::Serialize;

const FIXDATAEXCE_PATH: &str = "../../.resources/Romance_of_the_Three_Kingdoms_14/0010_SC/fixdataexce.s14";

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
            area::INDEX => {
                let records = table.records::<area::RecordArea>();
                insta::assert_snapshot!("area", write_to_csv(&records).expect("write area CSV"));
            }
            city::INDEX => {
                let records = table.records::<city::RecordCity>();
                insta::assert_snapshot!("city", write_to_csv(&records).expect("write city CSV"));
            }
            pass::INDEX => {
                let records = table.records::<pass::RecordPass>();
                insta::assert_snapshot!("pass", write_to_csv(&records).expect("write pass CSV"));
            }
            table4::INDEX => {
                let records = table.records::<table4::RecordTable4>();
                insta::assert_snapshot!("table4", write_to_csv(&records).expect("write table4 CSV"));
            }
            table5::INDEX => {
                assert_eq!(table.count, 0x34);
                assert!(table.bytes.is_empty());
            }
            table6::INDEX => {
                assert_eq!(table.count, 0xBD10);
                assert_eq!(table.bytes.len(), table.count * table6::SIZE);
            }
            table7::INDEX => assert_eq!(table.bytes.len(), table.count * table7::SIZE),
            table8::INDEX => assert!(table.bytes.is_empty()),
            table9::INDEX => assert_eq!(table.bytes.len(), table.count * table9::SIZE),
            table10::INDEX => assert_eq!(table.bytes.len(), table.count * table10::SIZE),
            table11::INDEX => assert!(table.bytes.is_empty()),
            table12::INDEX => assert_eq!(table.bytes.len(), table.count * table12::SIZE),
            table13::INDEX => assert_eq!(table.bytes.len(), table.count * table13::SIZE),
            _ => println!("table 0x{:02X}: {} bytes", table.index, table.bytes.len()),
        }
    }
}
