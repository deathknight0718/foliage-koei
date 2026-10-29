mod commons;

use koei_san14::domain::msggame::MsgGame;

const PATH: &str = "msggame";

#[test]
fn test_unpack() -> anyhow::Result<()> {
    let bytes = bytes!(PATH, ".bin")?;
    let data = MsgGame::try_from(bytes.as_slice()).expect("parse msggame.bin");
    std::fs::write(path!(PATH, ".bin.unpack"), data.data)?;
    Ok(())
}

#[test]
fn test_pack() -> anyhow::Result<()> {
    let bytes = bytes!(PATH, ".bin.unpack")?;
    let data = MsgGame::try_from(bytes.as_slice()).expect("parse msggame.bin");
    let compressed: Vec<u8> = data.try_into()?;
    std::fs::write(path!(PATH, ".bin.pack"), compressed)?;
    Ok(())
}
