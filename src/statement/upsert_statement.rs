use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};
use rmp_serde::{encode, decode};
use std::collections::HashMap;
use crate::protocol::MessageType;
use crate::statement::validate::validate_alphanumunderscore;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct UpsertStatement {
    #[validate(custom(function = "validate_alphanumunderscore"))]
    #[serde(rename = "table_name")]
    pub table_name: String,

    #[validate(length(min = 1))]
    #[serde(rename = "values")]
    pub values: HashMap<String, serde_json::Value>,

    #[serde(rename = "unique_key")]
    pub unique_key: String,
}

impl UpsertStatement {
    pub fn new(table_name: String, values: HashMap<String, serde_json::Value>, unique_key: String) -> Result<Self, ValidationErrors> {
        let stmt = UpsertStatement { table_name, values, unique_key };
        stmt.validate()?;
        Ok(stmt)
    }

    pub fn protocol(&self) -> MessageType {
        MessageType::Upsert
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, encode::Error> {
        encode::to_vec(self)
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self, decode::Error> {
        decode::from_slice(data)
    }

    pub fn to_string(&self) -> String {
        format!(
            "UpsertStatement{{TableName: {}, Values: {:?}, UniqueKey: {}}}",
            self.table_name, self.values, self.unique_key
        )
    }
}
