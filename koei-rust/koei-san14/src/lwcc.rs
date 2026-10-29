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
use anyhow::{Context, Result};
use std::cmp::Reverse;

// ----------------------------------------------------------------------------

pub const LWC_MAGIC: u32 = u32::from_le_bytes(*b"LWC\x1A");
pub const LWC_HEAD_SIZE: usize = 0x10C;
const DICTIONARY_SIZE: usize = 0x100;
const MIN_MATCH: usize = 3;

pub const SYMBOL_BASE: [usize; 32] = {
    let mut bases = [0; 32];
    let mut length = 1;
    while length < bases.len() {
        bases[length] = (1usize << length) - 2;
        length += 1;
    }
    bases
};

struct BitReader<'a> {
    bytes: &'a [u8],
    byte: u8,
    remaining: u8,
}

impl<'a> BitReader<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, byte: 0, remaining: 0 }
    }

    fn read_bit(&mut self) -> Result<usize> {
        if self.remaining == 0 {
            let (&byte, rest) = self.bytes.split_first().context("LWC bitstream ended before the declared output length")?;
            self.bytes = rest;
            self.byte = byte;
            self.remaining = 8;
        }
        self.remaining -= 1;
        Ok(usize::from((self.byte >> self.remaining) & 1))
    }

    fn read_symbol(&mut self) -> Result<usize> {
        let mut length = 1;
        while self.read_bit()? != 0 {
            length += 1;
            anyhow::ensure!(length < SYMBOL_BASE.len(), "LWC symbol prefix exceeds the maximum length");
        }
        let mut value = 0;
        for _ in 0..length {
            value = (value << 1) | self.read_bit()?;
        }
        Ok(SYMBOL_BASE[length] + value)
    }
}

// ----------------------------------------------------------------------------

struct BitWriter {
    bytes: Vec<u8>,
    available: u8,
}

impl BitWriter {
    fn new() -> Self {
        Self { bytes: Vec::new(), available: 0 }
    }

    fn write_bits(&mut self, value: usize, length: usize) {
        for shift in (0..length).rev() {
            if self.available == 0 {
                self.bytes.push(0);
                self.available = 8;
            }
            self.available -= 1;
            *self.bytes.last_mut().expect("bit writer always has a current byte") |= u8::from((value >> shift) & 1 == 1) << self.available;
        }
    }

    fn write_symbol(&mut self, symbol: usize) -> Result<()> {
        let length = symbol.checked_add(2).context("LWC symbol cannot be encoded")?.ilog2() as usize;
        anyhow::ensure!(length < SYMBOL_BASE.len(), "LWC symbol cannot be encoded");
        self.write_bits((1usize << length) - 2, length);
        self.write_bits(symbol - SYMBOL_BASE[length], length);
        Ok(())
    }

    fn into_inner(self) -> Vec<u8> {
        self.bytes
    }
}

// ----------------------------------------------------------------------------

pub fn decompress(bytes: &[u8], offset: usize) -> Result<Vec<u8>> {
    let bytes = bytes.get(offset..).context("LWC payload is missing or incomplete")?;
    let header = bytes.get(..LWC_HEAD_SIZE).context("LWC header is missing or incomplete")?;
    let magic = u32::from_le_bytes(header[..4].try_into()?);
    anyhow::ensure!(magic == LWC_MAGIC, "expected LWC magic 0x{LWC_MAGIC:X}, found 0x{magic:X}");
    let size = u32::from_le_bytes(header[4..8].try_into()?) as usize;
    let dictionary = &header[12..];
    let mut reader = BitReader::new(&bytes[LWC_HEAD_SIZE..]);
    let mut output = Vec::with_capacity(size);
    while output.len() < size {
        let symbol = reader.read_symbol()?;
        if symbol < dictionary.len() {
            output.push(dictionary[symbol]);
            continue;
        }
        let distance = symbol - dictionary.len();
        anyhow::ensure!(distance > 0 && distance <= output.len(), "LWC back-reference distance {distance} cannot be zero or exceed {} decoded bytes", output.len());
        let length = reader.read_symbol()?.checked_add(MIN_MATCH).ok_or_else(|| anyhow::anyhow!("LWC match length overflows while decoding symbol {symbol}"))?;
        anyhow::ensure!(length <= size - output.len(), "LWC back-reference length {length} exceeds the remaining decoded size {}", size - output.len());
        for _ in 0..length {
            let source = output.len() - distance;
            output.push(output[source]);
        }
    }
    Ok(output)
}

fn symbol_bit_length(symbol: usize) -> usize {
    2 * (symbol + 2).ilog2() as usize
}

fn match_hash(bytes: &[u8]) -> usize {
    (usize::from(bytes[0]) * 251 * 251 + usize::from(bytes[1]) * 251 + usize::from(bytes[2])) & 0xFFFF
}

struct MatchFinder<'a> {
    payload: &'a [u8],
    literal_bits: Vec<u64>,
    heads: Vec<usize>,
    previous: Vec<usize>,
}

impl<'a> MatchFinder<'a> {
    fn new(payload: &'a [u8], symbols: &[usize; DICTIONARY_SIZE]) -> Result<Self> {
        let mut literal_bits = Vec::with_capacity(payload.len().checked_add(1).context("LWC payload is too large to index")?);
        let mut bits = 0u64;
        literal_bits.push(bits);
        for &byte in payload {
            bits += symbol_bit_length(symbols[usize::from(byte)]) as u64;
            literal_bits.push(bits);
        }
        Ok(Self { payload, literal_bits, heads: vec![usize::MAX; 0x10000], previous: vec![usize::MAX; payload.len()] })
    }

    fn find(&self, position: usize) -> Option<(usize, usize)> {
        const MAX_CANDIDATES: usize = 128;
        const MAX_DISTANCE: usize = u32::MAX as usize - 2 - DICTIONARY_SIZE;
        if self.payload.len() - position < MIN_MATCH {
            return None;
        }

        let mut candidate = self.heads[match_hash(&self.payload[position..])];
        let mut best = None;
        let mut best_saving = 0;
        for _ in 0..MAX_CANDIDATES {
            if candidate == usize::MAX || position - candidate > MAX_DISTANCE {
                break;
            }
            let distance = position - candidate;
            let mut length = 0;
            while position + length < self.payload.len() && self.payload[candidate + length] == self.payload[position + length] {
                length += 1;
            }
            if length >= MIN_MATCH {
                let cost = (symbol_bit_length(DICTIONARY_SIZE + distance) + symbol_bit_length(length - MIN_MATCH)) as u64;
                let literals = self.literal_bits[position + length] - self.literal_bits[position];
                if literals > cost && literals - cost > best_saving {
                    best = Some((distance, length));
                    best_saving = literals - cost;
                }
                if position + length == self.payload.len() {
                    break;
                }
            }
            candidate = self.previous[candidate];
        }
        best
    }

    fn insert(&mut self, start: usize, end: usize) {
        for index in start..end {
            if self.payload.len() - index < MIN_MATCH {
                break;
            }
            let hash = match_hash(&self.payload[index..]);
            self.previous[index] = self.heads[hash];
            self.heads[hash] = index;
        }
    }
}

/// Encodes an LWC payload using literals and back-references.
pub fn compress(bytes: &[u8], offset: usize) -> Result<Vec<u8>> {
    let head = bytes.get(..offset).context("LWC prefix is missing or incomplete")?;
    let payload = bytes.get(offset..).context("LWC payload is missing")?;
    let size = u32::try_from(payload.len()).context("LWC payload exceeds the maximum encodable size")?;
    let mut header = [0; LWC_HEAD_SIZE];
    header[..4].copy_from_slice(&LWC_MAGIC.to_le_bytes());
    header[4..8].copy_from_slice(&size.to_le_bytes());
    let mut frequencies = [0usize; DICTIONARY_SIZE];
    for &byte in payload {
        frequencies[usize::from(byte)] += 1;
    }
    let dictionary = &mut header[12..];
    for (index, byte) in dictionary.iter_mut().enumerate() {
        *byte = index as u8;
    }
    dictionary.sort_by_key(|&byte| Reverse(frequencies[usize::from(byte)]));
    let mut symbols = [0usize; DICTIONARY_SIZE];
    for (index, &byte) in dictionary.iter().enumerate() {
        symbols[usize::from(byte)] = index;
    }
    let mut matches = MatchFinder::new(payload, &symbols)?;
    let mut writer = BitWriter::new();
    let mut position = 0;
    while position < payload.len() {
        let consumed = if let Some((distance, length)) = matches.find(position) {
            writer.write_symbol(DICTIONARY_SIZE + distance)?;
            writer.write_symbol(length - MIN_MATCH)?;
            length
        } else {
            writer.write_symbol(symbols[usize::from(payload[position])])?;
            1
        };
        matches.insert(position, position + consumed);
        position += consumed;
    }
    let bitstream = writer.into_inner();
    let compressed_size = u32::try_from(bitstream.len()).context("LWC compressed payload exceeds the maximum encodable size")?;
    header[8..12].copy_from_slice(&compressed_size.to_le_bytes());
    let mut compressed = Vec::with_capacity(head.len() + header.len() + bitstream.len());
    compressed.extend_from_slice(head);
    compressed.extend_from_slice(&header);
    compressed.extend_from_slice(&bitstream);
    Ok(compressed)
}
