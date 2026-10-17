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
pub mod area;
pub mod city;
pub mod pass;
pub mod table10;
pub mod table11;
pub mod table12;
pub mod table13;
pub mod table4;
pub mod table5;
pub mod table6;
pub mod table7;
pub mod table8;
pub mod table9;

use anyhow::{Result, anyhow};
use binrw::BinRead;
use serde::Serializer;
use std::io::{Cursor, Read};

// ----------------------------------------------------------------------------

pub struct Table {
    pub flag: [u8; 16],
    pub count: usize,
    pub size: usize,
    pub index: u32,
    pub bytes: Vec<u8>,
}

impl Table {
    pub fn read(cursor: &mut Cursor<&[u8]>, flag: [u8; 16]) -> Result<Option<Self>> {
        let offset = cursor.position() as usize;
        let index = u32::read_le(cursor)?;
        let Some(size) = record_size(&flag, index) else {
            return Ok(None);
        };
        let count = u32::read_le(cursor)? as usize;
        let length = count.checked_mul(size).ok_or_else(|| anyhow!("decoded record range overflows at offset 0x{offset:X}"))?;
        let mut bytes = vec![0; length];
        cursor.read_exact(&mut bytes)?;
        Ok(Some(Self { flag, count, size, index, bytes }))
    }
    pub fn records<T: Record>(&self) -> Vec<T> {
        if self.size == 0 {
            return std::iter::repeat_with(|| T::from(&[])).take(self.count).collect();
        }
        assert_eq!(self.bytes.len() % self.size, 0, "table byte length is not divisible by record size");
        self.bytes.chunks_exact(self.size).map(T::from).collect()
    }
}

pub fn se_utf16<S: Serializer>(bytes: &[u8], serializer: S) -> core::result::Result<S::Ok, S::Error> {
    let units = bytes.chunks_exact(2).map(|chunk| u16::from_le_bytes(chunk.try_into().expect("UTF-16LE unit size"))).collect::<Vec<_>>();
    serializer.serialize_str(String::from_utf16_lossy(&units).trim_end_matches('\0'))
}

pub fn se_u8<S: Serializer>(bytes: &[u8; 1], serializer: S) -> core::result::Result<S::Ok, S::Error> {
    serializer.serialize_u8(bytes[0])
}

pub fn se_hex<S: Serializer>(bytes: &[u8], serializer: S) -> core::result::Result<S::Ok, S::Error> {
    let value = bytes.iter().map(|byte| format!("{byte:02X}")).collect::<String>();
    serializer.serialize_str(&value)
}

// ----------------------------------------------------------------------------

pub trait Record: for<'a> From<&'a [u8]> {}

// ----------------------------------------------------------------------------

pub fn record_size(flag: &[u8; 16], index: u32) -> Option<usize> {
    match (flag, index) {
        (flag, area::INDEX) if flag == area::FLAG => Some(area::SIZE),
        (flag, city::INDEX) if flag == city::FLAG => Some(city::SIZE),
        (flag, pass::INDEX) if flag == pass::FLAG => Some(pass::SIZE),
        (flag, table4::INDEX) if flag == table4::FLAG => Some(table4::SIZE),
        (flag, table5::INDEX) if flag == table5::FLAG => Some(table5::SIZE),
        (flag, table6::INDEX) if flag == table6::FLAG => Some(table6::SIZE),
        (flag, table7::INDEX) if flag == table7::FLAG => Some(table7::SIZE),
        (flag, table8::INDEX) if flag == table8::FLAG => Some(table8::SIZE),
        (flag, table9::INDEX) if flag == table9::FLAG => Some(table9::SIZE),
        (flag, table10::INDEX) if flag == table10::FLAG => Some(table10::SIZE),
        (flag, table11::INDEX) if flag == table11::FLAG => Some(table11::SIZE),
        (flag, table12::INDEX) if flag == table12::FLAG => Some(table12::SIZE),
        (flag, table13::INDEX) if flag == table13::FLAG => Some(table13::SIZE),
        _ => None,
    }
}
