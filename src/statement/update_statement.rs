use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};
use rmp_serde::{encode, decode};
use std::collections::HashMap;
use crate::protocol::MessageType;
use crate::statement::validate::validate_alphanumunderscore;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct UpdateStatement {
    #[validate(custom(function = "validate_alphanumunderscore"))]
    #[serde(rename = "table_name")]
    pub table_name: String,

    #[validate(length(min = 1))]
    #[serde(rename = "updates")]
    pub updates: HashMap<String, serde_json::Value>,

    #[serde(rename = "where")]
    pub where_clause: String,
}

impl UpdateStatement {
    pub fn new(table_name: String, updates: HashMap<String, serde_json::Value>, where_clause: String) -> Result<Self, ValidationErrors> {
        let stmt = UpdateStatement { table_name, updates, where_clause };
        stmt.validate()?;
        Ok(stmt)
    }

    pub fn protocol(&self) -> MessageType {
        MessageType::Update
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, encode::Error> {
        encode::to_vec(self)
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self, decode::Error> {
        decode::from_slice(data)
    }

    pub fn to_string(&self) -> String {
        format!("UpdateStatement{{TableName: {}, Updates: {:?}, Where: {}}}", self.table_name, self.updates, self.where_clause)
    }
}
