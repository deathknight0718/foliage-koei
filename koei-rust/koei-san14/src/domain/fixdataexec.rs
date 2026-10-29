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
use super::{Decoder, Encoder};
use crate::AsAnyhowResult;
use crate::fixc::{Row, Skip, Table};
use crate::lwcc;
use crate::{se_t16, se_u08};
use anyhow::Result;
use binrw::{BinRead, BinWrite};
use serde::Serialize;
use std::io::Cursor;

// ----------------------------------------------------------------------------

pub const HEAD_SIZE: usize = 0x02F2;

// ----------------------------------------------------------------------------

#[repr(C)]
#[derive(BinRead, BinWrite)]
#[brw(little)]
pub struct FileHex {
    pub head: [u8; HEAD_SIZE],
    pub tab00: Table<Skip<0x0014>>,
    pub tab01: Table<Skip<0x0020>>,
    pub tab02: Table<Skip<0x0035>>,
    pub tab03: Table<Skip<0x0028>>,
    pub tab04: Table<Skip<0x0002>>,
    pub tab05: Table<Skip<0x0000>>,
    pub tab06: Table<Skip<0x0003>>,
    pub tab07: Table<Area>,
    pub tab08: Table<Skip<0x0000>>,
    pub tab09: Table<Skip<0x0033>>,
    pub tab0a: Table<Skip<0x0035>>,
    pub tab0b: Table<Skip<0x0000>>,
    pub tab0c: Table<Skip<0x0098>>,
    pub tab0d: Table<Skip<0x005E>>,
    pub tab0e: Table<Skip<0x0016>>,
    pub tab0f: Table<Skip<0x007F>>,
    pub tab10: Table<Skip<0x00A6>>,
    pub tab11: Table<Skip<0x002E>>,
    pub tab12: Table<Skip<0x0066>>,
    pub tab13: Table<Skip<0x0094>>,
    pub tab14: Table<Skip<0x0015>>,
    pub tab15: Table<Skip<0x000D>>,
    pub tab16: Table<Skip<0x0000>>,
    pub tab17: Table<Personality>,
    pub tab18: Table<Skip<0x0000>>,
    pub tab19: Table<Skip<0x0000>>,
    pub tab1a: Table<Skip<0x0000>>,
    pub tab1b: Table<Skip<0x0086>>,
    pub tab1c: Table<Skip<0x0000>>,
    pub tab1d: Table<Skip<0x000B>>,
    pub tab1e: Table<Skip<0x001A>>,
    pub tab1f: Table<Skip<0x0000>>,
    pub tab20: Table<Skip<0x0000>>,
    pub tab21: Table<Skip<0x000E>>,
    pub tab22: Table<Skip<0x0000>>,
    pub tab23: Table<Skip<0x0000>>,
    pub tab24: Table<Skip<0x00A8>>,
    pub tab25: Table<Skip<0x0019>>,
    pub tab26: Table<Skip<0x0035>>,
    pub tab27: Table<Skip<0x0038>>,
    pub tab28: Table<Skip<0x0004>>,
    pub tab29: Table<Skip<0x0022>>,
    pub tab2a: Table<Skip<0x0021>>,
    pub tab2b: Table<Skip<0x0027>>,
    pub tab2c: Table<Skip<0x0000>>,
}

impl Decoder for FileHex {
    fn decode(bytes: &[u8]) -> Result<Self> {
        let head = bytes.get(..HEAD_SIZE).ok_or_else(|| anyhow::anyhow!("FileHex header requires 0x{:X} bytes, found 0x{:X}", HEAD_SIZE, bytes.len()))?;
        let decompressed = lwcc::decompress(bytes, head.len())?;
        let mut combined = Vec::with_capacity(head.len() + decompressed.len());
        combined.extend_from_slice(head);
        combined.extend_from_slice(&decompressed);
        let mut cursor = Cursor::new(combined.as_slice());
        FileHex::read(&mut cursor).as_anyhow_result()
    }
}

impl Encoder for FileHex {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut buffer = Cursor::new(Vec::new());
        self.write(&mut buffer).as_anyhow_result()?;
        lwcc::compress(&buffer.into_inner(), HEAD_SIZE)
    }
}

impl TryFrom<&[u8]> for FileHex {
    type Error = anyhow::Error;

    fn try_from(bytes: &[u8]) -> Result<Self> {
        Self::decode(bytes)
    }
}

impl TryInto<Vec<u8>> for FileHex {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Vec<u8>> {
        self.encode()
    }
}

// ----------------------------------------------------------------------------

#[derive(BinRead, BinWrite, Serialize)]
#[brw(little)]
/// 0x0041 区域表
pub struct Area {
    #[serde(serialize_with = "se_t16")]
    pub lp00: [u8; 0x0A],
    #[serde(serialize_with = "se_t16")]
    pub lp01: [u8; 0x1A],
    #[serde(serialize_with = "se_u08")]
    pub lp02: [u8; 0x01],
    #[serde(skip_serializing)]
    pub lp03: [u8; 0x02],
    #[serde(skip_serializing)]
    pub lp04: [u8; 0x02],
    #[serde(skip_serializing)]
    pub lp05: [u8; 0x02],
    #[serde(skip_serializing)]
    pub lp06: [u8; 0x02],
    #[serde(skip_serializing)]
    pub lp07: [u8; 0x02],
    #[serde(skip_serializing)]
    pub lp08: [u8; 0x10],
    #[serde(skip_serializing)]
    pub lp09: [u8; 0x01],
    #[serde(skip_serializing)]
    pub lp0a: [u8; 0x01],
}

impl Row for Area {}

// ----------------------------------------------------------------------------

#[derive(BinRead, BinWrite, Serialize)]
#[brw(little)]
pub struct Personality {
    /// 名称 UTF-16LE
    #[serde(serialize_with = "se_t16")]
    pub lp00: [u8; 0x0C],
    /// 拼音 UTF-16LE
    #[serde(serialize_with = "se_t16")]
    pub lp01: [u8; 0x20],
    /// 备注 UTF-16LE
    #[serde(serialize_with = "se_t16")]
    pub lp02: [u8; 0x68],
    /// 分隔符
    #[serde(skip_serializing)]
    pub lp03: [u8; 0x04],
    /// 效果 1
    #[serde(serialize_with = "se_u08")]
    pub lp04: [u8; 0x01],
    /// 效果 2
    #[serde(serialize_with = "se_u08")]
    pub lp05: [u8; 0x01],
    /// 未知
    #[serde(skip_serializing)]
    pub lp06: [u8; 0x10],
}

impl Row for Personality {}
