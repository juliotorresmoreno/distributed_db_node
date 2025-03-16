use serde::{ Deserialize, Serialize };
use validator::{ Validate, ValidationErrors };
use rmp_serde::{ encode, decode };
use crate::statement::validate::validate_alphanumunderscore;
use crate::protocol::MessageType;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct AlterTableStatement {
    #[validate(custom(function = "validate_alphanumunderscore"))]
    #[serde(rename = "table_name")]
    pub table_name: String,

    #[validate(length(min = 1))]
    #[serde(rename = "changes")]
    pub changes: String,
}

impl AlterTableStatement {
    pub fn new(table_name: String, changes: String) -> Result<Self, ValidationErrors> {
        let stmt = AlterTableStatement { table_name, changes };
        stmt.validate()?;
        Ok(stmt)
    }

    pub fn protocol(&self) -> MessageType {
        MessageType::AlterTable
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, encode::Error> {
        encode::to_vec(self)
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self, decode::Error> {
        decode::from_slice(data)
    }

    pub fn to_string(&self) -> String {
        format!("AlterTableStatement{{TableName: {}, Changes: {}}}", self.table_name, self.changes)
    }
}
