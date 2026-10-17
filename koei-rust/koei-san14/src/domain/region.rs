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

pub const FLAG: &[u8; 16] = b"SN14SCEXVER0001\0";
pub const SIZE: usize = 0x28;
pub const INDEX: u32 = 0x02;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RecordRegion {
    pub lp00: [u8; 08], // 0x00..0x07：名称 UTF-16LE
    pub lp08: [u8; 20], // 0x08..0x1B：拼音 UTF-16LE
    pub lp1c: [u8; 10], // 0x1C..0x25：未知
    pub lp26: u8,       // 0x26：未知
    pub lp27: u8,       // 0x27：区域 ID
}

impl Record for RecordRegion {
    fn flag() -> &'static [u8; 16] {
        FLAG
    }
    fn index() -> u32 {
        INDEX
    }
}

impl From<&[u8]> for RecordRegion {
    fn from(bytes: &[u8]) -> Self {
        Self {
            lp00: bytes[0x00..0x08].try_into().expect("record length was checked before parsing"), //
            lp08: bytes[0x08..0x1C].try_into().expect("record length was checked before parsing"),
            lp1c: bytes[0x1C..0x26].try_into().expect("record length was checked before parsing"),
            lp26: bytes[0x26],
            lp27: bytes[0x27],
        }
    }
}
