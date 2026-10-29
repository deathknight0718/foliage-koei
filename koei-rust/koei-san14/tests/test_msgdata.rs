mod commons;

use koei_san14::domain::msgdata::MsgData;

const PATH: &str = "msgdata";

#[test]
fn test_unpack() -> anyhow::Result<()> {
    let bytes = bytes!(PATH, ".bin")?;
    let data = MsgData::try_from(bytes.as_slice()).expect("parse msgdata.bin");
    std::fs::write(path!(PATH, ".bin.unpack"), data.data)?;
    Ok(())
}

#[test]
fn test_pack() -> anyhow::Result<()> {
    let bytes = bytes!(PATH, ".bin.unpack")?;
    let data = MsgData::try_from(bytes.as_slice()).expect("parse msgdata.bin");
    let compressed: Vec<u8> = data.try_into()?;
    std::fs::write(path!(PATH, ".bin.pack"), compressed)?;
    Ok(())
}
