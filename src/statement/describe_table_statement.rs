use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};
use rmp_serde::{encode, decode};
use crate::protocol::MessageType;
use crate::statement::validate::validate_alphanumunderscore;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct DescribeTableStatement {
    #[validate(custom(function = "validate_alphanumunderscore"))]
    #[serde(rename = "table_name")]
    pub table_name: String,
}

impl DescribeTableStatement {
    pub fn new(table_name: String) -> Result<Self, ValidationErrors> {
        let stmt = DescribeTableStatement { table_name };
        stmt.validate()?;
        Ok(stmt)
    }

    pub fn protocol(&self) -> MessageType {
        MessageType::DescribeTable
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, encode::Error> {
        encode::to_vec(self)
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self, decode::Error> {
        decode::from_slice(data)
    }

    pub fn to_string(&self) -> String {
        format!("DescribeTableStatement{{TableName: {}}}", self.table_name)
    }
}
