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
use anyhow::Result;
use anyhow::{anyhow, bail};
use binrw::BinReaderExt;
use std::io::Cursor;

// ----------------------------------------------------------------------------

pub const LWC_MAGIC: u32 = u32::from_le_bytes(*b"LWC\x1A");
pub const LWC_HEAD_SIZE: usize = 0x10C;
pub const LWC_OFFSET: usize = 0x2F2;

// ----------------------------------------------------------------------------

pub const SYMBOL_BASE: [usize; 32] = [
    0x00000000, //
    0x00000000, //
    0x00000002, //
    0x00000006, //
    0x0000000E, //
    0x0000001E, //
    0x0000003E, //
    0x0000007E, //
    0x000000FE, //
    0x000001FE, //
    0x000003FE, //
    0x000007FE, //
    0x00000FFE, //
    0x00001FFE, //
    0x00003FFE, //
    0x00007FFE, //
    0x0000FFFE, //
    0x0001FFFE, //
    0x0003FFFE, //
    0x0007FFFE, //
    0x000FFFFE, //
    0x001FFFFE, //
    0x003FFFFE, //
    0x007FFFFE, //
    0x00FFFFFE, //
    0x01FFFFFE, //
    0x03FFFFFE, //
    0x07FFFFFE, //
    0x0FFFFFFE, //
    0x1FFFFFFE, //
    0x3FFFFFFE, //
    0x7FFFFFFE, //
];

// ----------------------------------------------------------------------------

fn read_length(prefix: u8) -> usize {
    match prefix {
        0x00..=0x7F => 1,
        0x80..=0xBF => 2,
        0xC0..=0xDF => 3,
        0xE0..=0xEF => 4,
        0xF0..=0xF7 => 5,
        0xF8..=0xFB => 6,
        0xFC..=0xFD => 7,
        0xFE => 8,
        0xFF => 9,
    }
}

fn read_byte(cursor: &mut Cursor<&[u8]>) -> Result<usize> {
    Ok(usize::from(BinReaderExt::read_le::<u8>(cursor).map_err(|_| anyhow!("LWC bitstream ended before the declared output length"))?))
}

fn read_symbol(cursor: &mut Cursor<&[u8]>, code: &mut usize, available: &mut usize) -> Result<usize> {
    *code &= 0xFF;
    let mut carried = 0;
    let mut length = read_length(*code as u8);
    if *available < length {
        loop {
            *code = read_byte(cursor)?;
            carried += *available;
            length = read_length(*code as u8);
            *available = 8;
            if length <= 8 {
                break;
            }
        }
    }
    let total = carried + length;
    let remaining = *available - length;
    *code = (*code << length) & 0xFF;
    let value = if remaining < total {
        let required = total - remaining;
        let mut value = *code >> (8 - remaining);
        let mut byte = read_byte(cursor)?;
        if required > 8 {
            let mut whole_bytes = (required - 9) / 8 + 1;
            let tail = required - whole_bytes * 8;
            while whole_bytes != 0 {
                value = (value << 8) | byte;
                byte = read_byte(cursor)?;
                whole_bytes -= 1;
            }
            *available = 8 - tail;
            *code = byte << tail;
            (*code >> 8) | (value << tail)
        } else {
            *available = 8 - required;
            *code = byte << required;
            (*code >> 8) | (value << required)
        }
    } else {
        *available = remaining - total;
        *code <<= total;
        *code >> 8
    };
    SYMBOL_BASE.get(total).and_then(|base| base.checked_add(value)).ok_or_else(|| anyhow!("LWC symbol {value} cannot be decoded"))
}

// ----------------------------------------------------------------------------

pub fn decompress(bytes: &[u8]) -> Result<Vec<u8>> {
    if bytes.len() < LWC_HEAD_SIZE {
        bail!("LWC header requires 0x{LWC_HEAD_SIZE:X} bytes, found 0x{:X}", bytes.len());
    }
    let magic = u32::from_le_bytes(bytes[..4].try_into().expect("checked LWC magic size"));
    if magic != LWC_MAGIC {
        bail!("expected LWC magic 0x{LWC_MAGIC:X}, found 0x{magic:X}");
    }
    let output_length = u32::from_le_bytes(bytes[4..8].try_into().expect("checked LWC length size")) as usize;
    let dictionary = &bytes[12..12 + 0x100];
    let mut cursor = Cursor::new(&bytes[LWC_HEAD_SIZE..]);
    let mut code = 0;
    let mut available = 0;
    let mut output = Vec::with_capacity(output_length);
    while output.len() < output_length {
        let symbol = read_symbol(&mut cursor, &mut code, &mut available)?;
        if symbol < dictionary.len() {
            output.push(dictionary[symbol]);
            continue;
        }
        if symbol == dictionary.len() {
            bail!("LWC symbol {symbol} cannot be decoded");
        }
        let distance = symbol - dictionary.len();
        if distance == 0 || distance > output.len() {
            bail!("LWC back-reference distance {distance} exceeds {} decoded bytes", output.len());
        }
        let length = read_symbol(&mut cursor, &mut code, &mut available)?.checked_add(3).ok_or_else(|| anyhow!("LWC match length overflows while decoding symbol {symbol}"))?;
        for _ in 0..length {
            if output.len() == output_length {
                break;
            }
            let source = output.len() - distance;
            output.push(output[source]);
        }
    }
    Ok(output)
}
