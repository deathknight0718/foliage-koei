mod commons;

use binrw::{BinRead, BinWrite};
use koei_san14::domain::scedaex::FileHex;
use koei_san14::domain::{Decoder, Encoder};
use std::io::Cursor;

const NAME: &str = "scedaex01";

#[test]
fn test_tables() -> anyhow::Result<()> {
    let bytes = bytes!(NAME, ".s14")?;
    let data = FileHex::decode(bytes.as_slice()).expect("parse scedaex*.s14");
    snapshot!("personality", &data.tab16.rows);
    Ok(())
}

#[test]
fn test_decode() -> anyhow::Result<()> {
    let bytes = bytes!(NAME, ".s14")?;
    let data = FileHex::decode(bytes.as_slice()).expect("parse scedaex*.s14");
    let mut buffer = Cursor::new(Vec::new());
    data.write(&mut buffer)?;
    std::fs::write(path!(NAME, ".s14.unpack"), buffer.into_inner())?;
    Ok(())
}

#[test]
fn test_encode() -> anyhow::Result<()> {
    let bytes = bytes!(NAME, ".s14.unpack")?;
    let data = FileHex::read(&mut Cursor::new(bytes))?;
    std::fs::write(path!(NAME, ".s14.pack"), data.encode()?)?;
    Ok(())
}
