mod commons;

use binrw::{BinRead, BinWrite};
use koei_san14::domain::fixdata::FileHex;
use koei_san14::domain::{Decoder, Encoder};
use std::io::Cursor;

const NAME: &str = "fixdata";

#[test]
fn test_tables() -> anyhow::Result<()> {
    let bytes = bytes!(NAME, ".s14")?;
    let data = FileHex::decode(bytes.as_slice()).expect("parse fixdata.s14");
    snapshot!("area", &data.tab07.rows);
    snapshot!("personality", &data.tab17.rows);
    Ok(())
}

#[test]
fn test_decode() -> anyhow::Result<()> {
    let bytes = bytes!(NAME, ".s14")?;
    let data = FileHex::decode(bytes.as_slice()).expect("parse fixdata.s14");
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
