use anyhow::Result as AnyhowResult;
use koei_san14::domain::msgdata::MsgData;
use std::path::Path;

const PATH: &str = "../../.data/msgdata.bin";

#[test]
fn test_unpack() -> AnyhowResult<()> {
    let bytes = std::fs::read(PATH)?;
    let data = MsgData::try_from(bytes.as_slice()).expect("parse msgdata.bin");
    std::fs::write(Path::new("../../.data").join("msgdata.unpack"), data.data)?;
    Ok(())
}

#[test]
fn test_pack() -> AnyhowResult<()> {
    let bytes = std::fs::read(PATH)?;
    let data = MsgData::try_from(bytes.as_slice()).expect("parse msgdata.bin");
    let compressed: Vec<u8> = data.try_into()?;
    std::fs::write(Path::new("../../.data").join("msgdata.pack"), compressed)?;
    Ok(())
}
