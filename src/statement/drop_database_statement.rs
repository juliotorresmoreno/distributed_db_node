use serde::{ Deserialize, Serialize };
use validator::{ Validate, ValidationErrors };
use rmp_serde::{ encode, decode };
use crate::protocol::MessageType;
use crate::statement::Statement;
use crate::statement::validate::validate_alphanumunderscore;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct DropDatabaseStatement {
    #[validate(custom(function = "validate_alphanumunderscore"))]
    #[serde(rename = "database_name")]
    pub database_name: String,
}

impl Statement for DropDatabaseStatement {
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
    
    fn protocol(&self) -> MessageType {
        MessageType::DropDatabase
    }

    fn to_bytes(&self) -> Result<Vec<u8>, encode::Error> {
        encode::to_vec(self)
    }

    fn from_bytes(data: &[u8]) -> Result<Box<dyn Statement>, decode::Error> {
        let stmt: DropDatabaseStatement = decode::from_slice(data)?;
        Ok(Box::new(stmt))
    }

    fn to_string(&self) -> String {
        format!("DropDatabaseStatement{{DatabaseName: {}}}", self.database_name)
    }
}

impl DropDatabaseStatement {
    fn new(database_name: String) -> Result<Self, ValidationErrors> {
        let stmt = DropDatabaseStatement { database_name };
        stmt.validate()?;
        Ok(stmt)
    }
}
