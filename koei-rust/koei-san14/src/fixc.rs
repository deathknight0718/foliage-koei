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
use anyhow::Result;
use binrw::BinRead;
use std::io::{Cursor, Read};

// ----------------------------------------------------------------------------

pub const EXPECTED_HEADER_VALUE: u32 = 0x66;

const ENTRY_LIMIT: u32 = 0x0C;
const ENTRY_SIZE: usize = 0x14;

// ----------------------------------------------------------------------------

#[derive(BinRead)]
#[br(little, magic = 0x66_u32)]
struct FixdataHeader {
    flag: [u8; 16],
}

#[derive(BinRead)]
#[br(little, magic = 0x66_u32)]
struct FixdataDecodedHeader {
    count: u32,
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
        anyhow::ensure!(bytes.len() >= offset, "fixdataexce version 1 payload begins at 0x{offset:X}, found only 0x{:X} bytes", bytes.len());
        cursor.set_position(offset as u64);
        let decompressed = lwcc::decompress(&mut cursor)?;
        let mut cursor = Cursor::new(decompressed.as_slice());
        let count = FixdataDecodedHeader::read(&mut cursor)?.count;
        anyhow::ensure!(count <= ENTRY_LIMIT, "expected at most {ENTRY_LIMIT} leading records, found {count}");
        let mut entries = vec![0; count as usize * ENTRY_SIZE];
        cursor.read_exact(&mut entries)?;
        let mut tables = Vec::new();
        while let Some(table) = Table::parse(&mut cursor, flag)? {
            println!("table 0x{:02X}: {} bytes {} count", table.index, table.bytes.len(), table.count);
            tables.push(table);
        }
        Ok(Self { flag, tables })
    }
}
