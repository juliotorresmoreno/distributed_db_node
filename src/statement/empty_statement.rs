use serde::{Deserialize, Serialize};
use rmp_serde::{encode, decode};
use crate::protocol::MessageType;
use crate::statement::Statement;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmptyStatement {
    #[serde(rename = "message_type")]
    pub message_type: u32,
}

#[allow(dead_code)]
impl EmptyStatement {
    pub fn new(message_type: MessageType) -> Self {
        Self {
            message_type: message_type as u32,
        }
    }
}

impl Statement for EmptyStatement {
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }

    fn protocol(&self) -> MessageType {
        MessageType::from_id(self.message_type)
    }

    fn to_bytes(&self) -> Result<Vec<u8>, encode::Error> {
        encode::to_vec(self)
    }

    fn from_bytes(data: &[u8]) -> Result<Box<dyn Statement>, decode::Error> {
        let stmt: EmptyStatement = decode::from_slice(data)?;
        Ok(Box::new(stmt))
    }

    fn to_string(&self) -> String {
        format!("EmptyStatement{{MessageType: {}}}", self.protocol().to_name())
    }
}
