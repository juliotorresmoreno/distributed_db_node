use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::{Validate, ValidationErrors};
use rmp_serde::{encode, decode};
use crate::statement::validate_alphanumunderscore;
use crate::protocol::MessageType;
use crate::statement::Statement;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct InsertStatement {
    #[validate(custom(function = "validate_alphanumunderscore"))]
    #[serde(rename = "table_name")]
    pub table_name: String,

    #[serde(rename = "values")]
    #[validate(length(min = 1))]
    pub values: HashMap<String, serde_json::Value>,
}

#[allow(dead_code)]
impl InsertStatement {
    pub fn new(table_name: String, values: HashMap<String, serde_json::Value>) -> Result<Self, ValidationErrors> {
        let stmt = InsertStatement { table_name, values };
        stmt.validate()?;
        Ok(stmt)
    }
}

impl Statement for InsertStatement {
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }

    fn protocol(&self) -> MessageType {
        MessageType::Insert
    }

    fn to_bytes(&self) -> Result<Vec<u8>, encode::Error> {
        encode::to_vec(self)
    }

    fn from_bytes(data: &[u8]) -> Result<Box<dyn Statement>, decode::Error> {
        let stmt: InsertStatement = decode::from_slice(data)?;
        Ok(Box::new(stmt))
    }

    fn to_string(&self) -> String {
        format!("InsertStatement{{TableName: {}, Values: {:?}}}", self.table_name, self.values)
    }
}
