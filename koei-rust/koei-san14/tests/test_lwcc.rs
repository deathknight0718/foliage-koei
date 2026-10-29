use anyhow::Result as AnyhowResult;
use binrw::BinWrite;
use koei_san14::lwcc::decompress;
use std::io::Cursor;
use std::path::Path;

const NAME: &str = "configS_exce_SC";

#[test]
fn test_unpack() -> AnyhowResult<()> {
    let bytes = std::fs::read(Path::new("../../.data").join(format!("{}.s14", NAME)))?;
    let decompressed = decompress(&bytes, 0)?;
    let mut buffer = Cursor::new(Vec::new());
    decompressed.write(&mut buffer)?;
    std::fs::write(Path::new("../../.data").join(format!("{}.unpack", NAME)), buffer.into_inner())?;
    Ok(())
}
