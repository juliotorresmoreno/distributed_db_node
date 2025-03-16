use serde::{ Deserialize, Serialize };
use rmp_serde::{ encode, decode };
use crate::protocol::MessageType;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmptyStatement {
    #[serde(rename = "message_type")]
    pub message_type: u32,
}

impl EmptyStatement {
    pub fn new(message_type: MessageType) -> Self {
        return Self {
            message_type: message_type as u32,
        };
    }

    pub fn protocol(&self) -> MessageType {
        return MessageType::from_id(self.message_type);
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, encode::Error> {
        encode::to_vec(self)
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self, decode::Error> {
        decode::from_slice(data)
    }

    pub fn to_string(&self) -> String {
        format!("EmptyStatement{{MessageType: {:?}}}", self.message_type)
    }
}
