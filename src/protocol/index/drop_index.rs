use serde::{ Serialize, Deserialize };
use rmp_serde::{ to_vec_named, from_slice };
use std::error::Error;
use crate::protocol::statement::{ Statement, MessageType };

#[derive(Debug, Serialize, Deserialize)]
pub struct DropIndexStatement {
    #[serde(rename = "index_name")]
    pub index_name: String,

    #[serde(rename = "table_name")]
    pub table_name: String,
}

impl DropIndexStatement {
    #[allow(dead_code)]
    pub fn new(index_name: &str, table_name: &str) -> Self {
        Self {
            index_name: index_name.to_string(),
            table_name: table_name.to_string(),
        }
    }
}

impl Statement for DropIndexStatement {
    fn protocol(&self) -> MessageType {
        MessageType::DropIndex
    }

    /// Serializes the statement into length-prefixed MessagePack bytes
    fn to_bytes(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        // Serialize to MessagePack bytes using named fields for readability
        let msgpack_bytes = to_vec_named(self)?;

        // Prefix the MessagePack bytes with the length (4 bytes)
        let length = (msgpack_bytes.len() as u32).to_be_bytes();
        let mut prefixed_bytes = Vec::with_capacity(4 + msgpack_bytes.len());
        prefixed_bytes.extend_from_slice(&length);
        prefixed_bytes.extend_from_slice(&msgpack_bytes);

        Ok(prefixed_bytes)
    }

    /// Deserializes the statement from length-prefixed MessagePack bytes
    fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        // Check if the bytes are long enough to contain a length prefix
        if bytes.len() < 4 {
            return Err("Invalid MessagePack data: not enough bytes for length prefix".into());
        }

        // Extract MessagePack data
        let msgpack_data = &bytes[4..];

        // Deserialize the MessagePack bytes
        let stmt: DropIndexStatement = from_slice(msgpack_data)?;
        Ok(stmt)
    }
}
