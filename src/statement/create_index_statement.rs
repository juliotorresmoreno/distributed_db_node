use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};
use rmp_serde::{encode, decode};
use crate::statement::validate::validate_alphanumunderscore;
use crate::protocol::MessageType;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct CreateIndexStatement {
    #[validate(custom(function = "validate_alphanumunderscore"))]
    #[serde(rename = "index_name")]
    pub index_name: String,

    #[validate(custom(function = "validate_alphanumunderscore"))]
    #[serde(rename = "table_name")]
    pub table_name: String,

    #[validate(length(min = 1))]
    #[serde(rename = "columns")]
    pub columns: Vec<String>,
}

impl CreateIndexStatement {
    pub fn new(index_name: String, table_name: String, columns: Vec<String>) -> Result<Self, ValidationErrors> {
        let stmt = CreateIndexStatement { index_name, table_name, columns };
        stmt.validate()?;
        Ok(stmt)
    }

    pub fn protocol(&self) -> MessageType {
        MessageType::CreateIndex
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, encode::Error> {
        encode::to_vec(self)
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self, decode::Error> {
        decode::from_slice(data)
    }

    pub fn to_string(&self) -> String {
        format!("CreateIndexStatement{{IndexName: {}, TableName: {}, Columns: {:?}}}", self.index_name, self.table_name, self.columns)
    }
}
