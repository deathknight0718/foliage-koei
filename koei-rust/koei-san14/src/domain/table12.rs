use super::Record;
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

pub const INDEX: u32 = 0x0B;
pub const FLAG: &[u8; 16] = b"SN14SCEXVER0001\0";
pub const SIZE: usize = 0x98;

#[derive(BinRead, BinWrite, Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[brw(little)]
pub struct RecordTable12 {
    pub lp00: [u8; 6],
    pub lp06: [u8; 14],
    pub lp14: [u8; 8],
    pub lp1c: [u8; 32],
    pub lp3c: [u8; 32],
    pub lp5c: [u8; 18],
    pub lp6e: [u8; 1],
    pub lp6f: [u8; 1],
    pub lp70: [u8; 2],
    pub lp72: [u8; 4],
    pub lp76: [u8; 1],
    pub lp77: [u8; 18],
    pub lp89: [u8; 2],
    pub lp8b: [u8; 10],
    pub lp95: [u8; 1],
    pub lp96: [u8; 1],
    pub lp97: [u8; 1],
}

impl Record for RecordTable12 {}

impl From<&[u8]> for RecordTable12 {
    fn from(bytes: &[u8]) -> Self {
        Self::read_le(&mut Cursor::new(bytes)).expect("record length was checked before parsing")
    }
}
