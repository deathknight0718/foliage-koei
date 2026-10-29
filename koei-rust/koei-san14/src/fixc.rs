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
use binrw::{BinRead, BinWrite};
use serde::Serialize;

// ----------------------------------------------------------------------------

pub trait Row: 'static + for<'a> BinRead<Args<'a> = ()> + for<'a> BinWrite<Args<'a> = ()> {}

// ----------------------------------------------------------------------------

#[derive(BinRead, BinWrite, Serialize)]
#[brw(little)]
pub struct Skip<const N: usize> {
    #[serde(skip_serializing)]
    pub lp00: [u8; N],
}

impl<const N: usize> Row for Skip<N> {}

// ----------------------------------------------------------------------------

#[derive(BinRead, BinWrite)]
#[brw(little)]
pub struct Table<T: Row> {
    pub id: u32,
    #[bw(assert(*size as usize == rows.len()))]
    pub size: u32,
    #[br(count = size)]
    pub rows: Vec<T>,
}
