use super::Record;
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

pub const INDEX: u32 = 0x0A;
pub const FLAG: &[u8; 16] = b"SN14SCEXVER0001\0";
pub const SIZE: usize = 0;

#[derive(BinRead, BinWrite, Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[brw(little)]
pub struct RecordTable11 {}

impl Record for RecordTable11 {}

impl From<&[u8]> for RecordTable11 {
    fn from(bytes: &[u8]) -> Self {
        Self::read_le(&mut Cursor::new(bytes)).expect("record length was checked before parsing")
    }
}
