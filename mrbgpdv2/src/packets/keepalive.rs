use bytes::BytesMut;

use super::header::{Header, MessageType};
use crate::error::ConvertBytesToBgpMessageError;

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub struct KeepaliveMessage {
    header: Header,
}

impl KeepaliveMessage {
    pub fn new() -> Self {
        let header = Header::new(19, MessageType::Keepalive);
        Self { header }
    }
}

impl Default for KeepaliveMessage {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<BytesMut> for KeepaliveMessage {
    type Error = ConvertBytesToBgpMessageError;

    fn try_from(bytes: BytesMut) -> Result<Self, Self::Error> {
        let header = Header::try_from(BytesMut::from(&bytes[0..19]))?;
        if header.type_ != MessageType::Keepalive {
            return Err(anyhow::anyhow!("type is not for keepalive").into());
        }
        Ok(Self { header })
    }
}

impl From<KeepaliveMessage> for BytesMut {
    fn from(keepalive: KeepaliveMessage) -> Self {
        keepalive.header.into()
    }
}