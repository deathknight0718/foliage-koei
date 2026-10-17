use super::Record;
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

pub const INDEX: u32 = 0x0C;
pub const FLAG: &[u8; 16] = b"SN14SCEXVER0001\0";
pub const SIZE: usize = 0x5E;

#[derive(BinRead, BinWrite, Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[brw(little)]
pub struct RecordTable13 {
    pub lp00: [u8; 10],
    pub lp0a: [u8; 26],
    pub lp24: [u8; 12],
    pub lp30: [u8; 18],
    pub lp42: [u8; 10],
    pub lp4c: [u8; 10],
    pub lp56: [u8; 8],
}

impl Record for RecordTable13 {}

impl From<&[u8]> for RecordTable13 {
    fn from(bytes: &[u8]) -> Self {
        Self::read_le(&mut Cursor::new(bytes)).expect("record length was checked before parsing")
    }
}
