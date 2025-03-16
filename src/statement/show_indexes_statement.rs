use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};
use rmp_serde::{encode, decode};
use crate::protocol::MessageType;
use crate::statement::validate::validate_alphanumunderscore;
use crate::statement::Statement;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct ShowIndexesStatement {
    #[validate(custom(function = "validate_alphanumunderscore"))]
    #[serde(rename = "table_name")]
    pub table_name: String,
}

impl ShowIndexesStatement {
    pub fn new(table_name: String) -> Result<Self, ValidationErrors> {
        let stmt = ShowIndexesStatement { table_name };
        stmt.validate()?;
        Ok(stmt)
    }
}

impl Statement for ShowIndexesStatement {
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }

    fn protocol(&self) -> MessageType {
        MessageType::ShowIndexes
    }

    fn to_bytes(&self) -> Result<Vec<u8>, encode::Error> {
        encode::to_vec(self)
    }

    fn from_bytes(data: &[u8]) -> Result<Box<dyn Statement>, decode::Error> {
        let stmt: ShowIndexesStatement = decode::from_slice(data)?;
        Ok(Box::new(stmt))
    }

    fn to_string(&self) -> String {
        format!("ShowIndexesStatement{{TableName: {}}}", self.table_name)
    }
}
