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
pub const SIZE: usize = 0x35;
pub const INDEX: u32 = 0x01;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RecordCity {
    pub lp00: [u8; 08], // 0x00..0x07：名称 UTF-16LE
    pub lp08: [u8; 20], // 0x08..0x1B：拼音 UTF-16LE
    pub lp1c: [u8; 10], // 0x1C..0x25：未知
    pub lp26: u8,       // 0x26：未知
    pub lp27: u8,       // 0x27：城市 ID
    pub lp28: [u8; 04], // 0x28..0x2B：未知
    pub lp2c: [u8; 04], // 0x2C..0x2F：未知
    pub lp30: [u8; 04], // 0x30..0x33：未知
    pub lp34: u8,       // 0x34：州 ID
}

impl Record for RecordCity {
    fn flag() -> &'static [u8; 16] {
        FLAG
    }
    fn index() -> u32 {
        INDEX
    }
}

impl From<&[u8]> for RecordCity {
    fn from(bytes: &[u8]) -> Self {
        Self {
            lp00: bytes[0x00..0x08].try_into().expect("record length was checked before parsing"), //
            lp08: bytes[0x08..0x1C].try_into().expect("record length was checked before parsing"),
            lp1c: bytes[0x1C..0x26].try_into().expect("record length was checked before parsing"),
            lp26: bytes[0x26],
            lp27: bytes[0x27],
            lp28: bytes[0x28..0x2C].try_into().expect("record length was checked before parsing"),
            lp2c: bytes[0x2C..0x30].try_into().expect("record length was checked before parsing"),
            lp30: bytes[0x30..0x34].try_into().expect("record length was checked before parsing"),
            lp34: bytes[0x34],
        }
    }
}
