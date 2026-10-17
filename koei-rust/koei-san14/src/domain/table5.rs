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
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

pub const INDEX: u32 = 0x04;
pub const FLAG: &[u8; 16] = b"SN14SCEXVER0001\0";
pub const SIZE: usize = 0;

#[derive(BinRead, BinWrite, Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[brw(little)]
pub struct RecordTable5 {}

impl Record for RecordTable5 {}

impl From<&[u8]> for RecordTable5 {
    fn from(bytes: &[u8]) -> Self {
        Self::read_le(&mut Cursor::new(bytes)).expect("record length was checked before parsing")
    }
}
