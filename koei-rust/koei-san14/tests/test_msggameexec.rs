use anyhow::Result as AnyhowResult;
use koei_san14::domain::msggameexec::MsgGameExec;
use std::path::Path;

const PATH: &str = "../../.data/msggame_exce.bin";

#[test]
fn test_unpack() -> AnyhowResult<()> {
    let bytes = std::fs::read(PATH)?;
    let data = MsgGameExec::try_from(bytes.as_slice()).expect("parse msggame_exce.bin");
    std::fs::write(Path::new("../../.data").join("msggame_exce.unpack"), data.data)?;
    Ok(())
}

#[test]
fn test_pack() -> AnyhowResult<()> {
    let bytes = std::fs::read(PATH)?;
    let data = MsgGameExec::try_from(bytes.as_slice()).expect("parse msggame_exce.bin");
    let compressed: Vec<u8> = data.try_into()?;
    std::fs::write(Path::new("../../.data").join("msggame_exce.pack"), compressed)?;
    Ok(())
}
