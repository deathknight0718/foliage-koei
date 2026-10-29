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
pub mod domain;
pub mod fixc;
pub mod lwcc;
pub mod msgc;

// ----------------------------------------------------------------------------

pub trait AsAnyhowResult<T> {
    fn as_anyhow_result(self) -> anyhow::Result<T>;
}

impl<T, E: std::error::Error + Send + Sync + 'static> AsAnyhowResult<T> for Result<T, E> {
    fn as_anyhow_result(self) -> anyhow::Result<T> {
        self.map_err(|e| anyhow::anyhow!(e))
    }
}

// ----------------------------------------------------------------------------

pub fn se_t16<S: serde::Serializer>(bytes: &[u8], serializer: S) -> core::result::Result<S::Ok, S::Error> {
    let units = bytes.chunks_exact(2).map(|chunk| u16::from_le_bytes(chunk.try_into().expect("UTF-16LE unit size"))).collect::<Vec<_>>();
    serializer.serialize_str(String::from_utf16_lossy(&units).trim_end_matches('\0'))
}

pub fn se_u08<S: serde::Serializer>(bytes: &[u8; 1], serializer: S) -> core::result::Result<S::Ok, S::Error> {
    serializer.serialize_u8(bytes[0])
}

pub fn se_u16<S: serde::Serializer>(bytes: &[u8; 2], serializer: S) -> core::result::Result<S::Ok, S::Error> {
    serializer.serialize_u16(u16::from_le_bytes(*bytes))
}

pub fn se_u32<S: serde::Serializer>(bytes: &[u8; 4], serializer: S) -> core::result::Result<S::Ok, S::Error> {
    serializer.serialize_u32(u32::from_le_bytes(*bytes))
}

pub fn se_hex<S: serde::Serializer>(bytes: &[u8], serializer: S) -> core::result::Result<S::Ok, S::Error> {
    let value = bytes.iter().map(|byte| format!("{byte:02X}")).collect::<String>();
    serializer.serialize_str(&value)
}
