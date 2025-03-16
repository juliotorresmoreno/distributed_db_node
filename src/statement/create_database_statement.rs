use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};
use rmp_serde::{encode, decode};
use crate::statement::validate::validate_alphanumunderscore;
use crate::protocol::MessageType;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct CreateDatabaseStatement {
    #[validate(custom(function = "validate_alphanumunderscore"))]
    #[serde(rename = "database_name")]
    pub database_name: String,
}

impl CreateDatabaseStatement {
    pub fn new(database_name: String) -> Result<Self, ValidationErrors> {
        let stmt = CreateDatabaseStatement { database_name };
        stmt.validate()?;
        Ok(stmt)
    }

    pub fn protocol(&self) -> MessageType {
        MessageType::CreateDatabase
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, encode::Error> {
        encode::to_vec(self)
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self, decode::Error> {
        decode::from_slice(data)
    }

    pub fn to_string(&self) -> String {
        format!("CreateDatabaseStatement{{DatabaseName: {}}}", self.database_name)
    }
}
