use std::{error::Error, fmt};

pub const HEADER_SIZE: usize = 20;
pub const EXPECTED_HEADER_VALUE: u32 = 0x66;
pub const VERSION_TAG: &[u8; 16] = b"SN14SCEXVER0001\0";

#[derive(Debug, PartialEq, Eq)]
pub struct FixDataExce<'a> {
    pub header_value: u32,
    pub payload: &'a [u8],
}

#[derive(Debug, PartialEq, Eq)]
pub enum FixDataExceError {
    Truncated { actual: usize },
    UnexpectedHeaderValue { actual: u32 },
    UnexpectedVersionTag,
}

impl fmt::Display for FixDataExceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Truncated { actual } => write!(
                formatter,
                "fixdataexce header requires {HEADER_SIZE} bytes, found {actual}"
            ),
            Self::UnexpectedHeaderValue { actual } => write!(
                formatter,
                "expected fixdataexce header value 0x{EXPECTED_HEADER_VALUE:X}, found 0x{actual:X}"
            ),
            Self::UnexpectedVersionTag => {
                formatter.write_str("expected version tag SN14SCEXVER0001")
            }
        }
    }
}

impl Error for FixDataExceError {}

impl<'a> FixDataExce<'a> {
    pub fn parse(bytes: &'a [u8]) -> Result<Self, FixDataExceError> {
        if bytes.len() < HEADER_SIZE {
            return Err(FixDataExceError::Truncated {
                actual: bytes.len(),
            });
        }

        let header_value = u32::from_le_bytes(bytes[..4].try_into().expect("checked header size"));
        if header_value != EXPECTED_HEADER_VALUE {
            return Err(FixDataExceError::UnexpectedHeaderValue {
                actual: header_value,
            });
        }

        if bytes[4..HEADER_SIZE] != *VERSION_TAG {
            return Err(FixDataExceError::UnexpectedVersionTag);
        }

        Ok(Self {
            header_value,
            payload: &bytes[HEADER_SIZE..],
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EncoderSection {
    pub identifier: Option<u32>,
    pub element_count: u32,
    pub memory_record_size: Option<usize>,
}

const CONFIRMED_PREFIX_SECTIONS: [EncoderSection; 10] = [
    EncoderSection {
        identifier: None,
        element_count: 0x0C,
        memory_record_size: None,
    },
    EncoderSection {
        identifier: Some(0),
        element_count: 0x17,
        memory_record_size: None,
    },
    EncoderSection {
        identifier: Some(1),
        element_count: 0x34,
        memory_record_size: Some(0x168),
    },
    EncoderSection {
        identifier: Some(2),
        element_count: 0x0B,
        memory_record_size: Some(0xC0),
    },
    EncoderSection {
        identifier: Some(3),
        element_count: 0x34,
        memory_record_size: Some(0x1D0),
    },
    EncoderSection {
        identifier: Some(4),
        element_count: 0x34,
        memory_record_size: None,
    },
    EncoderSection {
        identifier: Some(5),
        element_count: 0xBD10,
        memory_record_size: Some(0x20),
    },
    EncoderSection {
        identifier: Some(6),
        element_count: 0x1F5,
        memory_record_size: Some(0x98),
    },
    EncoderSection {
        identifier: Some(7),
        element_count: 0xBB9,
        memory_record_size: Some(0x20),
    },
    EncoderSection {
        identifier: Some(8),
        element_count: 0x1F,
        memory_record_size: Some(0x48),
    },
];

const CONFIRMED_TAIL_SECTIONS: [EncoderSection; 2] = [
    EncoderSection {
        identifier: Some(11),
        element_count: 0x15,
        memory_record_size: Some(0xC0),
    },
    EncoderSection {
        identifier: Some(12),
        element_count: 0xC9,
        memory_record_size: Some(0x88),
    },
];

pub fn confirmed_prefix_sections() -> &'static [EncoderSection] {
    &CONFIRMED_PREFIX_SECTIONS
}

pub fn confirmed_tail_sections() -> &'static [EncoderSection] {
    &CONFIRMED_TAIL_SECTIONS
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> Vec<u8> {
        let mut bytes = Vec::from(EXPECTED_HEADER_VALUE.to_le_bytes());
        bytes.extend_from_slice(VERSION_TAG);
        bytes.extend_from_slice(&[0xAA, 0xBB]);
        bytes
    }

    #[test]
    fn parses_confirmed_header() {
        let bytes = fixture();
        let document = FixDataExce::parse(&bytes).unwrap();

        assert_eq!(document.header_value, 0x66);
        assert_eq!(document.payload, [0xAA, 0xBB]);
    }

    #[test]
    fn rejects_fixdata_header() {
        let mut bytes = fixture();
        bytes[..4].copy_from_slice(&0x33_u32.to_le_bytes());

        assert_eq!(
            FixDataExce::parse(&bytes),
            Err(FixDataExceError::UnexpectedHeaderValue { actual: 0x33 })
        );
    }

    #[test]
    fn exposes_confirmed_encoder_sections() {
        assert_eq!(confirmed_prefix_sections()[0].element_count, 0x0C);
        assert_eq!(confirmed_prefix_sections()[6].identifier, Some(5));
        assert_eq!(confirmed_tail_sections()[1].memory_record_size, Some(0x88));
    }
}
