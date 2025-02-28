use serde::{ Serialize, Deserialize };
use rmp_serde::{ to_vec_named, from_slice };
use std::error::Error;
use crate::protocol::statement::{ Statement, MessageType };

#[derive(Debug, Serialize, Deserialize)]
pub struct DescribeTableStatement {
    #[serde(rename = "table_name")]
    pub table_name: String,
}

impl DescribeTableStatement {
    pub fn new(table_name: &str) -> Self {
        Self {
            table_name: table_name.to_string(),
        }
    }
}

impl Statement for DescribeTableStatement {
    fn protocol(&self) -> MessageType {
        MessageType::DescribeTable
    }

    fn to_bytes(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        let msgpack_bytes = to_vec_named(self)?;

        let length = (msgpack_bytes.len() as u32).to_be_bytes();
        let mut prefixed_bytes = Vec::with_capacity(4 + msgpack_bytes.len());
        prefixed_bytes.extend_from_slice(&length);
        prefixed_bytes.extend_from_slice(&msgpack_bytes);

        Ok(prefixed_bytes)
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        if bytes.len() < 4 {
            return Err("Invalid MessagePack data: not enough bytes for length prefix".into());
        }

        let msgpack_data = &bytes[4..];

        let stmt: DescribeTableStatement = from_slice(msgpack_data)?;
        Ok(stmt)
    }
}
