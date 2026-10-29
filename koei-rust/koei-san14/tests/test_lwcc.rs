mod commons;

use binrw::BinWrite;
use koei_san14::lwcc::decompress;
use std::io::Cursor;

const NAME: &str = "configS_exce_SC";

#[test]
fn test_decode() -> anyhow::Result<()> {
    let bytes = bytes!(NAME, ".s14")?;
    let decompressed = decompress(&bytes, 0)?;
    let mut buffer = Cursor::new(Vec::new());
    decompressed.write(&mut buffer)?;
    std::fs::write(path!(NAME, ".s14.unpack"), buffer.into_inner())?;
    Ok(())
}
