use anyhow::Result;

use crate::msgc;

pub struct MsgDataExec {
    pub data: Vec<u8>,
}

impl TryFrom<&[u8]> for MsgDataExec {
    type Error = anyhow::Error;

    fn try_from(bytes: &[u8]) -> Result<Self> {
        Ok(Self { data: msgc::decode(bytes)? })
    }
}

impl TryInto<Vec<u8>> for MsgDataExec {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Vec<u8>> {
        msgc::encode(&self.data)
    }
}
