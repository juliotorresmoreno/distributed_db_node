use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};
use rmp_serde::{encode, decode};
use crate::protocol::MessageType;
use crate::statement::{Statement, validate::validate_alphanumunderscore};

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct TruncateTableStatement {
    #[validate(custom(function = "validate_alphanumunderscore"))]
    #[serde(rename = "table_name")]
    pub table_name: String,
}

impl TruncateTableStatement {
    pub fn new(table_name: String) -> Result<Self, ValidationErrors> {
        let stmt = TruncateTableStatement { table_name };
        stmt.validate()?;
        Ok(stmt)
    }
}

impl Statement for TruncateTableStatement {
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }

    fn protocol(&self) -> MessageType {
        MessageType::TruncateTable
    }

    fn to_bytes(&self) -> Result<Vec<u8>, encode::Error> {
        encode::to_vec(self)
    }

    fn from_bytes(data: &[u8]) -> Result<Box<dyn Statement>, decode::Error> {
        let stmt: TruncateTableStatement = decode::from_slice(data)?;
        Ok(Box::new(stmt))
    }

    fn to_string(&self) -> String {
        format!("TruncateTableStatement{{TableName: {}}}", self.table_name)
    }
}
