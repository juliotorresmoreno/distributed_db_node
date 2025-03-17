use tokio::net::TcpStream;
use std::time::{ SystemTime, UNIX_EPOCH };
use tokio::io::{ self, AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf };
use uuid::Uuid;
use byteorder::{ BigEndian, ReadBytesExt, WriteBytesExt };
use std::io::Write;
use crate::protocol::MessageType;
use crate::statement::Statement;
use super::{ MessageHeader, MessageTypeFlag, MESSAGE_HEADER_SIZE };

#[derive(Debug, Clone)]
pub struct Message {
    pub header: MessageHeader,
    pub body: Vec<u8>,
}

#[allow(dead_code)]
impl Message {
    pub fn new(message_type: MessageType, stmt: &impl Statement) -> Self {
        let body = stmt.to_bytes().unwrap();
        let body_size = body.len() as u32;
        Self {
            header: MessageHeader::new(message_type, MessageTypeFlag::RequestMessage, body_size),
            body,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer = self.header.serialize();
        buffer.extend_from_slice(&self.body);
        return buffer;
    }

    pub fn deserialize(buffer: &[u8]) -> io::Result<Self> {
        if buffer.len() < MESSAGE_HEADER_SIZE {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Message size too small"));
        }

        let header = MessageHeader::deserialize(&buffer[..MESSAGE_HEADER_SIZE])?;
        let body = buffer[MESSAGE_HEADER_SIZE..].to_vec();

        if (body.len() as u32) != header.body_size {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Body size mismatch"));
        }

        Ok(Self { header, body })
    }

    pub async fn read_from(reader: &mut ReadHalf<TcpStream>) -> io::Result<Self> {
        let mut header_bytes = vec![0; MESSAGE_HEADER_SIZE];
        reader.read_exact(&mut header_bytes).await?;

        let header = MessageHeader::deserialize(&header_bytes)?;
        let mut body = vec![0; header.body_size as usize];
        reader.read_exact(&mut body).await?;

        return Ok(Self { header, body });
    }

    pub async fn write_to(&self, writer: &mut WriteHalf<TcpStream>) -> io::Result<()> {
        let serialized = self.serialize();
        writer.write_all(&serialized).await?;
        return writer.flush().await;
    }
}
