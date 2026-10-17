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
pub const SIZE: usize = 0x20;
pub const INDEX: u32 = 0x00;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RecordArea {
    pub lp00: [u8; 10], // 0x00..0x09：名称 UTF-16LE
    pub lp0a: [u8; 14], // 0x0A..0x17：拼音 UTF-16LE
    pub lp18: [u8; 04], // 0x18..0x1B：显示名称 UTF-16LE
    pub lp1c: [u8; 02], // 0x1C..0x1D：未知，当前均为 00 00
    pub lp1e: u8,       // 0x1E：区域 ID
    pub lp1f: u8,       // 0x1F：地利 ID
}

impl Record for RecordArea {
    fn flag() -> &'static [u8; 16] {
        FLAG
    }
    fn index() -> u32 {
        INDEX
    }
}

impl From<&[u8]> for RecordArea {
    fn from(bytes: &[u8]) -> Self {
        Self {
            lp00: bytes[0x00..0x0A].try_into().expect("record length was checked before parsing"), //
            lp0a: bytes[0x0A..0x18].try_into().expect("record length was checked before parsing"),
            lp18: bytes[0x18..0x1C].try_into().expect("record length was checked before parsing"),
            lp1c: bytes[0x1C..0x1E].try_into().expect("record length was checked before parsing"),
            lp1e: bytes[0x1E],
            lp1f: bytes[0x1F],
        }
    }
}
