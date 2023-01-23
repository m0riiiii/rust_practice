use anyhow::{Context, Result};
use bytes::{BufMut, BytesMut};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use crate::bgp_type;
use crate::config::{Config, Mode};
use crate::error::CreateConnectionError;
use crate::packets::message::Message;

#[derive(Debug)]
pub struct Connection {
    conn: TcpStream,
    buffer: BytesMut,
}

impl Connection {
    pub async fn connect(config: &Config) -> Result<Self, CreateConnectionError> {
        let conn = match config.mode {
            Mode::Active => Self::connect_to_remote_peer(config).await,
            Mode::Passive => Self::wait_connection_from_remote_peer(config).await,
        }?;
        let buffer = BytesMut::with_capacity(1500);
        Ok(Self{
            conn,
            buffer,
        })
    }

    pub async fn send(&mut self, message: Message) {
        let bytes: BytesMut = message.into();
        self.conn.write_all(&bytes[..]).await;
    }

    pub async fn get_message(&mut self) -> Option<Message> {
        self.read_data_from_tcp_connection().await;
        let buffer = self.split_buffer_at_message_separator()?;
        Message::try_from(buffer).ok()
    }

    async fn connect_to_remote_peer(config: &Config) -> Result<TcpStream> {
        let bgp_port = 179;
        TcpStream::connect((config.remote_ip, bgp_port))
            .await
            .context(format!(
                "cannot connect to remote peer {0}:{1}",
                config.remote_ip, bgp_port
            ))
    }

    async fn wait_connection_from_remote_peer(config: &Config) -> Result<TcpStream> {
        let bgp_port = 179;
        let listener = TcpListener::bind((config.local_ip, bgp_port))
            .await
            .context(format!(
                "cannot bind {0}:{1}",
                config.local_ip, bgp_port
            ))?;
        Ok(listener
            .accept()
            .await
            .context(format!(
                "cannot completely get TCP Connection from remote on {0}: {1}.\
                it is very likely the TCP Connection request from the remote has not arrived!",
                config.local_ip, bgp_port
            ))?.0)
    }

    async fn read_data_from_tcp_connection(&mut self) {
        loop {
            let mut buf: Vec<u8> = vec![];
            match self.conn.try_read_buf(&mut buf) {
                Ok(0) => (),
                Ok(n) => self.buffer.put(&buf[..]),
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => break,
                Err(e) => panic!("error occured when reading data from tcp connection: {:?}", e),
            }
        }
    }

    fn split_buffer_at_message_separator(&mut self) -> Option<BytesMut> {
        let index = self.get_index_of_message_separatpr().ok()?;
        if self.buffer.len() < index {
            return None;
        }
        Some(self.buffer.split_to(index))
    }

    fn get_index_of_message_separatpr(&self) -> Result<usize> {
        let minimum_message_length = 19;
        if self.buffer.len() < 19 {
            return Err(anyhow::anyhow!("buffer does not contain separator data of message."));
        }
        Ok(u16::from_be_bytes([self.buffer[16], self.buffer[17]]) as usize)
    }
}