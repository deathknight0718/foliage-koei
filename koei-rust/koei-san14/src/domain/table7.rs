use super::Record;
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

pub const INDEX: u32 = 0x06;
pub const FLAG: &[u8; 16] = b"SN14SCEXVER0001\0";
pub const SIZE: usize = 0x41;

#[derive(BinRead, BinWrite, Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[brw(little)]
pub struct RecordTable7 {
    pub lp00: [u8; 10],
    pub lp0a: [u8; 26],
    pub lp24: [u8; 1],
    pub lp25: [u8; 2],
    pub lp27: [u8; 2],
    pub lp29: [u8; 2],
    pub lp2b: [u8; 2],
    pub lp2d: [u8; 2],
    pub lp2f: [u8; 16],
    pub lp3f: [u8; 1],
    pub lp40: [u8; 1],
}

impl Record for RecordTable7 {}

impl From<&[u8]> for RecordTable7 {
    fn from(bytes: &[u8]) -> Self {
        Self::read_le(&mut Cursor::new(bytes)).expect("record length was checked before parsing")
    }
}
