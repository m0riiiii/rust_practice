use std::net::Ipv4Addr;
use bytes::BytesMut;

use super::header::{self, Header, MessageType};
use super::keepalive::KeepaliveMessage;
use super::update::UpdateMessage;
use crate::bgp_type::AutonomousSystemNumber;
use crate::error::{
    ConvertBgpMessageToBytesError, ConvertBytesToBgpMessageError,
};
use crate::packets::open::OpenMessage;

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub enum Message {
    Open(OpenMessage),
    Keepalive(KeepaliveMessage),
    Update(UpdateMessage),
}

impl Message {
    pub fn new_open(
        my_as_number: AutonomousSystemNumber,
        my_ip_addr: Ipv4Addr,
    ) -> Self {
        Self::Open(OpenMessage::new(my_as_number, my_ip_addr))
    }

    pub fn new_keepalive() -> Self {
        Self::Keepalive(KeepaliveMessage::new())
    }
}

impl TryFrom<BytesMut> for Message {
    type Error = ConvertBytesToBgpMessageError;

    fn try_from(bytes: BytesMut) -> Result<Self, Self::Error> {
        let header_bytes_length = 19;

        if bytes.len() < header_bytes_length {
            return Err(Self::Error::from(anyhow::anyhow!(
                "Cannot convert from bytes to message beacause bytes length is shorter than header."
            )));
        }

        let header = 
            Header::try_from(BytesMut::from(&bytes[0..header_bytes_length]))?;
        match header.type_ {
            MessageType::Open => {
                Ok(Message::Open(OpenMessage::try_from(bytes)?))
            },
            MessageType::Keepalive => {
                Ok(Message::Keepalive(KeepaliveMessage::try_from(bytes)?))
            },
            MessageType::Update => {
                Ok(Message::Update(UpdateMessage::try_from(bytes)?))
            },
        }
    }
}

impl From<Message> for BytesMut {
    fn from(message: Message) -> Self {
        match message {
            Message::Open(open) => open.into(),
            Message::Keepalive(keepalive) => keepalive.into(),
            Message::Update(update) => update.into(),
        }
    }
}
