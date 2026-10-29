use anyhow::Result as AnyhowResult;
use koei_san14::domain::msgdataexec::MsgDataExec;
use std::path::Path;

const PATH: &str = "../../.data/msgdata_exce.bin";

#[test]
fn test_unpack() -> AnyhowResult<()> {
    let bytes = std::fs::read(PATH)?;
    let data = MsgDataExec::try_from(bytes.as_slice()).expect("parse msgdata_exce.bin");
    std::fs::write(Path::new("../../.data").join("msgdata_exce.unpack"), data.data)?;
    Ok(())
}

#[test]
fn test_pack() -> AnyhowResult<()> {
    let bytes = std::fs::read(PATH)?;
    let data = MsgDataExec::try_from(bytes.as_slice()).expect("parse msgdata_exce.bin");
    let compressed: Vec<u8> = data.try_into()?;
    std::fs::write(Path::new("../../.data").join("msgdata_exce.pack"), compressed)?;
    Ok(())
}
