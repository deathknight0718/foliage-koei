use super::Record;
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

pub const INDEX: u32 = 0x09;
pub const FLAG: &[u8; 16] = b"SN14SCEXVER0001\0";
pub const SIZE: usize = 0x35;

#[derive(BinRead, BinWrite, Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[brw(little)]
pub struct RecordTable10 {
    pub lp00: [u8; 10],
    pub lp0a: [u8; 18],
    pub lp1c: [u8; 1],
    pub lp1d: [u8; 2],
    pub lp1f: [u8; 2],
    pub lp21: [u8; 2],
    pub lp23: [u8; 1],
    pub lp24: [u8; 1],
    pub lp25: [u8; 6],
    pub lp2b: [u8; 1],
    pub lp2c: [u8; 1],
    pub lp2d: [u8; 1],
    pub lp2e: [u8; 4],
    pub lp32: [u8; 1],
    pub lp33: [u8; 1],
    pub lp34: [u8; 1],
}

impl Record for RecordTable10 {}

impl From<&[u8]> for RecordTable10 {
    fn from(bytes: &[u8]) -> Self {
        Self::read_le(&mut Cursor::new(bytes)).expect("record length was checked before parsing")
    }
}
