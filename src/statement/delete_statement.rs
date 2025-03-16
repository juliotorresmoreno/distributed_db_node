use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};
use rmp_serde::{encode, decode};
use crate::protocol::MessageType;
use crate::statement::validate::validate_alphanumunderscore;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct DeleteStatement {
    #[validate(custom(function = "validate_alphanumunderscore"))]
    #[serde(rename = "table_name")]
    pub table_name: String,

    #[serde(rename = "where")]
    pub r#where: Option<String>,
}

impl DeleteStatement {
    pub fn new(table_name: String, r#where: Option<String>) -> Result<Self, ValidationErrors> {
        let stmt = DeleteStatement { table_name, r#where };
        stmt.validate()?;
        Ok(stmt)
    }

    pub fn protocol(&self) -> MessageType {
        MessageType::Delete
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, encode::Error> {
        encode::to_vec(self)
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self, decode::Error> {
        decode::from_slice(data)
    }

    pub fn to_string(&self) -> String {
        format!("DeleteStatement{{TableName: {}, Where: {}}}", self.table_name, self.r#where.clone().unwrap_or_default())
    }
}
