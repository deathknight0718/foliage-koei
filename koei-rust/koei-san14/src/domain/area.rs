/*
 * MIT License
 *
 * Copyright (c) 2025 Foliage Develop Team
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
use super::Record;
use super::{se_t16, se_u08};
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

pub const INDEX: u32 = 0x06;
pub const FLAG: &[u8; 16] = b"SN14SCEXVER0001\0";
pub const SIZE: usize = 0x41;

#[derive(BinRead, BinWrite, Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[brw(little)]
pub struct RecordArea {
    #[serde(serialize_with = "se_t16")]
    pub lp00: [u8; 10],
    #[serde(serialize_with = "se_t16")]
    pub lp0a: [u8; 26],
    #[serde(serialize_with = "se_u08")]
    pub lp24: [u8; 1],
    #[serde(skip_serializing)]
    pub lp25: [u8; 2],
    #[serde(skip_serializing)]
    pub lp27: [u8; 2],
    #[serde(skip_serializing)]
    pub lp29: [u8; 2],
    #[serde(skip_serializing)]
    pub lp2b: [u8; 2],
    #[serde(skip_serializing)]
    pub lp2d: [u8; 2],
    #[serde(skip_serializing)]
    pub lp2f: [u8; 16],
    #[serde(skip_serializing)]
    pub lp3f: [u8; 1],
    #[serde(skip_serializing)]
    pub lp40: [u8; 1],
}

impl Record for RecordArea {}

impl From<&[u8]> for RecordArea {
    fn from(bytes: &[u8]) -> Self {
        Self::read_le(&mut Cursor::new(bytes)).expect("record length was checked before parsing")
    }
}
