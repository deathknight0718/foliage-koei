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
use crate::domain::{Record, record_size};
use crate::lwcc::{LWC_OFFSET, decompress_lwc};
use anyhow::{Result, anyhow, bail};

// ----------------------------------------------------------------------------

pub const SIZE_HEAD: usize = 20;
pub const EXPECTED_HEADER_VALUE: u32 = 0x66;
const LEADING_RECORD_COUNT: u32 = 0x0C;
const LEADING_RECORD_SIZE: usize = 0x14;

// ----------------------------------------------------------------------------

#[derive(Debug, PartialEq, Eq)]
pub struct Fixdata<'a> {
    pub val: u32,
    pub flag: [u8; 16],
    pub payload: &'a [u8],
}

impl<'a> Fixdata<'a> {
    pub fn parse<T: Record>(bytes: &'a [u8]) -> Result<Self> {
        if bytes.len() < SIZE_HEAD {
            bail!("fixdataexce header requires {SIZE_HEAD} bytes, found {}", bytes.len());
        }
        let val = u32::from_le_bytes(bytes[..4].try_into().expect("checked header size"));
        if val != EXPECTED_HEADER_VALUE {
            bail!("expected fixdataexce header value 0x{EXPECTED_HEADER_VALUE:X}, found 0x{val:X}");
        }
        let flag: [u8; 16] = bytes[4..SIZE_HEAD].try_into().expect("checked header size");
        if flag != *T::flag() {
            bail!("unexpected fixdata flag for table index {}", T::index());
        }
        Ok(Self { val, flag, payload: &bytes[SIZE_HEAD..] })
    }
    pub fn decode(&self) -> Result<DecodedFixdata> {
        let payload_offset = LWC_OFFSET.checked_sub(SIZE_HEAD).expect("compressed payload follows the container header");
        let compressed = self.payload.get(payload_offset..).ok_or_else(|| anyhow!("fixdataexce version 1 payload begins at 0x{LWC_OFFSET:X}, found only 0x{:X} bytes", self.payload.len() + SIZE_HEAD))?;
        Ok(DecodedFixdata { val: self.val, flag: self.flag, payload: decompress_lwc(compressed)? })
    }
    pub fn read<T: Record>(&self) -> Result<Vec<T>> {
        self.decode()?.read()
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug, PartialEq, Eq)]
pub struct DecodedFixdata {
    pub val: u32,
    pub flag: [u8; 16],
    pub payload: Vec<u8>,
}

impl DecodedFixdata {
    pub fn read<T: Record>(&self) -> Result<Vec<T>> {
        let mut reader = DecodedReader::new(&self.payload, self.flag);
        let decoded_head = reader.read_u32()?;
        if decoded_head != EXPECTED_HEADER_VALUE {
            bail!("expected decoded fixdataexce value 0x{EXPECTED_HEADER_VALUE:X}, found 0x{decoded_head:X}");
        }
        let leading_count = reader.read_u32()?;
        if leading_count != LEADING_RECORD_COUNT {
            bail!("expected {LEADING_RECORD_COUNT} leading records, found {leading_count}");
        }
        reader.skip_records(leading_count, LEADING_RECORD_SIZE)?;
        reader.read_table::<T>()
    }
}

// ----------------------------------------------------------------------------

struct DecodedReader<'a> {
    bytes: &'a [u8],
    flag: [u8; 16],
    offset: usize,
}

impl<'a> DecodedReader<'a> {
    fn new(bytes: &'a [u8], flag: [u8; 16]) -> Self {
        Self { bytes, flag, offset: 0 }
    }
    fn read_u32(&mut self) -> Result<u32> {
        let bytes = self.read_exact(4)?;
        Ok(u32::from_le_bytes(bytes.try_into().expect("checked u32 size")))
    }
    fn read_exact(&mut self, length: usize) -> Result<&'a [u8]> {
        let offset = self.offset;
        let end = offset.checked_add(length).ok_or_else(|| anyhow!("decoded payload length overflows at 0x{offset:X} while reading {length} bytes"))?;
        let bytes = self.bytes.get(offset..end).ok_or_else(|| anyhow!("decoded payload requires {length} bytes at 0x{offset:X}, but contains 0x{:X} bytes", self.bytes.len()))?;
        self.offset = end;
        Ok(bytes)
    }
    fn read_table<T: Record>(&mut self) -> Result<Vec<T>> {
        loop {
            let offset = self.offset;
            let index = self.read_u32()?;
            let count = self.read_u32()?;
            if index == T::index() {
                let size = record_size(T::flag(), T::index()).ok_or_else(|| anyhow!("unregistered target table index {} for flag {:?}", T::index(), T::flag()))?;
                let records = self.read_records(count, size)?;
                return Ok(records.chunks_exact(size).map(T::from).collect());
            }
            self.skip_records(count, record_size(&self.flag, index).ok_or_else(|| anyhow!("unknown table index {index} for flag {:?} at decoded offset 0x{offset:X}", self.flag))?)?;
        }
    }
    fn skip_records(&mut self, count: u32, record_size: usize) -> Result<()> {
        self.read_records(count, record_size).map(|_| ())
    }
    fn read_records(&mut self, count: u32, record_size: usize) -> Result<&'a [u8]> {
        let offset = self.offset;
        let length = (count as usize).checked_mul(record_size).ok_or_else(|| anyhow!("decoded record range overflows at offset 0x{offset:X}"))?;
        self.read_exact(length)
    }
}