use anyhow::Result as AnyhowResult;
use koei_san14::domain::msggame::MsgGame;
use std::path::Path;

const PATH: &str = "../../.data/msggame.bin";

#[test]
fn test_unpack() -> AnyhowResult<()> {
    let bytes = std::fs::read(PATH)?;
    let data = MsgGame::try_from(bytes.as_slice()).expect("parse msggame.bin");
    std::fs::write(Path::new("../../.data").join("msggame.unpack"), data.data)?;
    Ok(())
}

#[test]
fn test_pack() -> AnyhowResult<()> {
    let bytes = std::fs::read(PATH)?;
    let data = MsgGame::try_from(bytes.as_slice()).expect("parse msggame.bin");
    let compressed: Vec<u8> = data.try_into()?;
    std::fs::write(Path::new("../../.data").join("msggame.pack"), compressed)?;
    Ok(())
}
