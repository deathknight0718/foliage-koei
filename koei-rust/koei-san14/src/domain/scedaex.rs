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
use crate::{se_i16, se_t16, se_u08, se_u32};
use anyhow::Result;
use binrw::{BinRead, BinWrite};
use serde::Serialize;
use std::io::Cursor;

// ----------------------------------------------------------------------------

pub const HEAD_SIZE: usize = 0x02F2;
pub const SKIP_SIZE: usize = 0x0D02;

// ----------------------------------------------------------------------------

#[repr(C)]
#[derive(BinRead, BinWrite)]
#[brw(little)]
pub struct FileHex {
    pub head: [u8; HEAD_SIZE],
    pub skip: [u8; SKIP_SIZE],
    pub tab00: Table<Skip<0x0000>>,
    pub tab01: Table<Skip<0x0017>>,
    pub tab02: Table<Skip<0x0014>>,
    pub tab03: Table<Skip<0x0146>>,
    pub tab04: Table<Skip<0x0004>>,
    pub tab05: Table<Skip<0x0000>>,
    pub tab06: Table<Skip<0x0008>>,
    pub tab07: Table<Skip<0x0006>>,
    pub tab08: Table<Skip<0x0000>>,
    pub tab09: Table<Skip<0x0000>>,
    pub tab0a: Table<Skip<0x016B>>,
    pub tab0b: Table<Skip<0x0000>>,
    pub tab0c: Table<Skip<0x0000>>,
    pub tab0d: Table<Skip<0x0041>>,
    pub tab0e: Table<Skip<0x0000>>,
    pub tab0f: Table<Skip<0x0000>>,
    pub tab10: Table<Skip<0x0000>>,
    pub tab11: Table<Skip<0x0000>>,
    pub tab12: Table<Skip<0x0000>>,
    pub tab13: Table<Skip<0x000F>>,
    pub tab14: Table<Skip<0x0000>>,
    pub tab15: Table<Skip<0x0000>>,
    pub tab16: Table<Personality>,
    pub tab17: Table<Skip<0x0000>>,
    pub tab18: Table<Skip<0x0000>>,
    pub tab19: Table<Skip<0x0000>>,
    pub tab1a: Table<Skip<0x0000>>,
    pub tab1b: Table<Skip<0x0000>>,
    pub tab1c: Table<Skip<0x0000>>,
    pub tab1d: Table<Skip<0x0000>>,
    pub tab1e: Table<Skip<0x0000>>,
    pub tab1f: Table<Skip<0x00D5>>,
    pub tab20: Table<Skip<0x0000>>,
    pub tab21: Table<Skip<0x0000>>,
    pub tab22: Table<Skip<0x0000>>,
    pub tab23: Table<Skip<0x0000>>,
    pub tab24: Table<Skip<0x0005>>,
    pub tab25: Table<Skip<0x0000>>,
    pub tab26: Table<Skip<0x020D>>,
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
/// 个性表
pub struct Personality {
    /// 名称
    #[serde(serialize_with = "se_t16")]
    pub name: [u8; 0x0C],
    /// 拼音
    #[serde(serialize_with = "se_t16")]
    pub enunciation: [u8; 0x20],
    /// 条件 1
    #[serde(serialize_with = "se_u08")]
    pub cid1: [u8; 0x01],
    /// 条件 2
    #[serde(serialize_with = "se_u08")]
    pub cid2: [u8; 0x01],
    /// 条件值 1
    #[serde(serialize_with = "se_u32")]
    pub cval1: [u8; 0x04],
    /// 条件值 2
    #[serde(serialize_with = "se_u32")]
    pub cval2: [u8; 0x04],
    /// 效果 1
    #[serde(serialize_with = "se_u08")]
    pub eid1: [u8; 0x01],
    /// 效果值 1
    #[serde(serialize_with = "se_i16")]
    pub eval1: [u8; 0x02],
    /// 效果 2
    #[serde(serialize_with = "se_u08")]
    pub eid2: [u8; 0x01],
    /// 效果值 2
    #[serde(serialize_with = "se_i16")]
    pub eval2: [u8; 0x02],
}

impl Row for Personality {}
