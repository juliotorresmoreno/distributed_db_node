use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};
use rmp_serde::{encode, decode};
use crate::protocol::MessageType;
use crate::statement::validate::validate_alphanumunderscore;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct DropDatabaseStatement {
    #[validate(custom(function = "validate_alphanumunderscore"))]
    #[serde(rename = "database_name")]
    pub database_name: String,
}

impl DropDatabaseStatement {
    pub fn new(database_name: String) -> Result<Self, ValidationErrors> {
        let stmt = DropDatabaseStatement { database_name };
        stmt.validate()?;
        Ok(stmt)
    }

    pub fn protocol(&self) -> MessageType {
        MessageType::DropDatabase
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, encode::Error> {
        encode::to_vec(self)
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self, decode::Error> {
        decode::from_slice(data)
    }

    pub fn to_string(&self) -> String {
        format!("DropDatabaseStatement{{DatabaseName: {}}}", self.database_name)
    }
}
