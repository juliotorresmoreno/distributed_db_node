use std::io::{ self, Read, Write };
use std::net::TcpStream;
use std::time::{ SystemTime, UNIX_EPOCH };
use uuid::Uuid;
use byteorder::{ BigEndian, ReadBytesExt, WriteBytesExt };
use crate::protocol::MessageType;
use crate::statement::Statement;

const START_MARKER: u32 = 0xdeadbeef;
const END_MARKER: u32 = 0xbeefdead;
const MESSAGE_HEADER_SIZE: usize = 37;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum MessageTypeFlag {
    RequestMessage = 1,
    ResponseMessage = 2,
}

#[derive(Debug, Clone)]
pub struct MessageHeader {
    pub start_marker: u32,
    pub message_id: [u8; 16],
    pub message_type: MessageType,
    pub message_flag: MessageTypeFlag,
    pub timestamp: u32,
    pub body_size: u32,
    pub end_marker: u32,
}

impl MessageHeader {
    pub fn new(message_type: MessageType, message_flag: MessageTypeFlag, body_size: u32) -> Self {
        Self {
            start_marker: START_MARKER,
            message_id: Uuid::new_v4().as_bytes().clone(),
            message_type,
            message_flag,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u32,
            body_size,
            end_marker: END_MARKER,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(MESSAGE_HEADER_SIZE);
        buffer.write_u32::<BigEndian>(self.start_marker).unwrap();
        buffer.extend_from_slice(&self.message_id);
        buffer.write_u32::<BigEndian>(self.message_type as u32).unwrap();
        buffer.write_u8(self.message_flag as u8).unwrap();
        buffer.write_u32::<BigEndian>(self.timestamp).unwrap();
        buffer.write_u32::<BigEndian>(self.body_size).unwrap();
        buffer.write_u32::<BigEndian>(self.end_marker).unwrap();
        buffer
    }

    pub fn deserialize(mut buffer: &[u8]) -> io::Result<Self> {
        if buffer.len() != MESSAGE_HEADER_SIZE {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid header size"));
        }

        let start_marker = buffer.read_u32::<BigEndian>()?;
        if start_marker != START_MARKER {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid start marker"));
        }

        let mut message_id = [0u8; 16];
        buffer.read_exact(&mut message_id)?;

        let message_type = match buffer.read_u32::<BigEndian>() {
            Ok(value) => MessageType::from_id(value),
            Err(_) => {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid message type"));
            }
        };

        let message_flag = match buffer.read_u8()? {
            1 => MessageTypeFlag::RequestMessage,
            2 => MessageTypeFlag::ResponseMessage,
            _ => {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid message flag"));
            }
        };

        let timestamp = buffer.read_u32::<BigEndian>()?;
        let body_size = buffer.read_u32::<BigEndian>()?;
        let end_marker = buffer.read_u32::<BigEndian>()?;

        if end_marker != END_MARKER {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid end marker"));
        }

        Ok(Self {
            start_marker,
            message_id,
            message_type,
            message_flag,
            timestamp,
            body_size,
            end_marker,
        })
    }

    pub fn message_id_string(&self) -> String {
        Uuid::from_slice(&self.message_id).unwrap().to_string()
    }
}

#[derive(Debug, Clone)]
pub struct Message {
    pub header: MessageHeader,
    pub body: Vec<u8>,
}

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
        buffer
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

    pub fn read_from(stream: &mut TcpStream) -> io::Result<Self> {
        let mut header_bytes = [0u8; MESSAGE_HEADER_SIZE];
        stream.read_exact(&mut header_bytes)?;
        let header = MessageHeader::deserialize(&header_bytes)?;

        let mut body = vec![0; header.body_size as usize];
        stream.read_exact(&mut body)?;

        Ok(Self { header, body })
    }

    pub fn write_to(&self, stream: &mut TcpStream) -> io::Result<()> {
        stream.write_all(&self.serialize())?;
        stream.flush()
    }
}
