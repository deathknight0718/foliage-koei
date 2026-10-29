mod commons;

use koei_san14::domain::msggameexec::MsgGameExec;

const PATH: &str = "msggame_exce";

#[test]
fn test_unpack() -> anyhow::Result<()> {
    let bytes = bytes!(PATH, ".bin")?;
    let data = MsgGameExec::try_from(bytes.as_slice()).expect("parse msggame_exce.bin");
    std::fs::write(path!(PATH, ".bin.unpack"), data.data)?;
    Ok(())
}

#[test]
fn test_pack() -> anyhow::Result<()> {
    let bytes = bytes!(PATH, ".bin.unpack")?;
    let data = MsgGameExec::try_from(bytes.as_slice()).expect("parse msggame_exce.bin");
    let compressed: Vec<u8> = data.try_into()?;
    std::fs::write(path!(PATH, ".bin.pack"), compressed)?;
    Ok(())
}
