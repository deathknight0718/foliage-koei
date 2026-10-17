use super::Record;
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

pub const INDEX: u32 = 0x08;
pub const FLAG: &[u8; 16] = b"SN14SCEXVER0001\0";
pub const SIZE: usize = 0x33;

#[derive(BinRead, BinWrite, Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[brw(little)]
pub struct RecordTable9 {
    pub lp00: [u8; 8],
    pub lp08: [u8; 18],
    pub lp1a: [u8; 2],
    pub lp1c: [u8; 1],
    pub lp1d: [u8; 18],
    pub lp2f: [u8; 1],
    pub lp30: [u8; 1],
    pub lp31: [u8; 1],
    pub lp32: [u8; 1],
}

impl Record for RecordTable9 {}

impl From<&[u8]> for RecordTable9 {
    fn from(bytes: &[u8]) -> Self {
        Self::read_le(&mut Cursor::new(bytes)).expect("record length was checked before parsing")
    }
}
