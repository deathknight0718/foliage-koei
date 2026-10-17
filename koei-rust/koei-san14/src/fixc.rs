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
use crate::domain::Table;
use crate::lwcc;
use anyhow::{Result, anyhow, bail};
use binrw::BinRead;
use std::io::{Cursor, Read};

// ----------------------------------------------------------------------------

pub const EXPECTED_HEADER_VALUE: u32 = 0x66;

const LEADING_RECORD_COUNT: u32 = 0x0C;
const LEADING_RECORD_SIZE: usize = 0x14;

// ----------------------------------------------------------------------------

#[derive(BinRead)]
#[br(little, magic = 0x66_u32)]
struct FixdataHeader {
    flag: [u8; 16],
}

#[derive(BinRead)]
#[br(little, magic = 0x66_u32)]
struct DecodedHeader {
    leading_count: u32,
}

// ----------------------------------------------------------------------------

pub struct Fixdata {
    pub flag: [u8; 16],
    pub tables: Vec<Table>,
}

impl TryFrom<&[u8]> for Fixdata {
    type Error = anyhow::Error;

    fn try_from(bytes: &[u8]) -> Result<Self> {
        let mut cursor = Cursor::new(bytes);
        let flag = FixdataHeader::read(&mut cursor)?.flag;
        let offset = lwcc::LWC_OFFSET;
        let body = bytes.get(offset..).ok_or_else(|| anyhow!("fixdataexce version 1 payload begins at 0x{offset:X}, found only 0x{:X} bytes", bytes.len()))?;
        let payload = lwcc::decompress(body)?;
        let mut cursor = Cursor::new(payload.as_slice());
        let leading_count = DecodedHeader::read(&mut cursor)?.leading_count;
        if leading_count != LEADING_RECORD_COUNT {
            bail!("expected {LEADING_RECORD_COUNT} leading records, found {leading_count}");
        }
        let mut leading_records = vec![0; leading_count as usize * LEADING_RECORD_SIZE];
        cursor.read_exact(&mut leading_records)?;
        let mut tables = Vec::new();
        while let Some(table) = Table::read(&mut cursor, flag)? {
            tables.push(table);
        }
        Ok(Self { flag, tables })
    }
}
