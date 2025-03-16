use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};
use rmp_serde::{encode, decode};
use crate::protocol::MessageType;
use crate::statement::validate::validate_alphanumunderscore;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct RenameTableStatement {
    #[validate(custom(function = "validate_alphanumunderscore"))]
    #[serde(rename = "old_table_name")]
    pub old_table_name: String,

    #[validate(custom(function = "validate_alphanumunderscore"))]
    #[serde(rename = "new_table_name")]
    pub new_table_name: String,
}

impl RenameTableStatement {
    pub fn new(old_table_name: String, new_table_name: String) -> Result<Self, ValidationErrors> {
        let stmt = RenameTableStatement { old_table_name, new_table_name };
        stmt.validate()?;
        Ok(stmt)
    }

    pub fn protocol(&self) -> MessageType {
        MessageType::RenameTable
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, encode::Error> {
        encode::to_vec(self)
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self, decode::Error> {
        decode::from_slice(data)
    }

    pub fn to_string(&self) -> String {
        format!("RenameTableStatement{{OldTableName: {}, NewTableName: {}}}", self.old_table_name, self.new_table_name)
    }
}
