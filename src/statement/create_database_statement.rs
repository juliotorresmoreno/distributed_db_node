use serde::{ Deserialize, Serialize };
use validator::{ Validate, ValidationErrors };
use rmp_serde::{ encode, decode };
use crate::statement::{ Statement, validate_alphanumunderscore };
use crate::protocol::MessageType;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct CreateDatabaseStatement {
    #[validate(custom(function = "validate_alphanumunderscore"))]
    #[serde(rename = "database_name")]
    pub database_name: String,
}

#[allow(dead_code)]
impl CreateDatabaseStatement {
    pub fn new(database_name: String) -> Result<Self, ValidationErrors> {
        let stmt = CreateDatabaseStatement { database_name };
        stmt.validate()?;
        Ok(stmt)
    }
}

impl Statement for CreateDatabaseStatement {
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }

    fn protocol(&self) -> MessageType {
        MessageType::CreateDatabase
    }

    fn to_bytes(&self) -> Result<Vec<u8>, encode::Error> {
        encode::to_vec(self)
    }

    fn from_bytes(data: &[u8]) -> Result<Box<dyn Statement>, decode::Error> {
        let stmt: CreateDatabaseStatement = decode::from_slice(data)?;
        Ok(Box::new(stmt))
    }

    fn to_string(&self) -> String {
        format!("CreateDatabaseStatement{{DatabaseName: {}}}", self.database_name)
    }
}
