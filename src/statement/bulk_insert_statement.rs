use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};
use rmp_serde::{encode, decode};
use crate::statement::{ validate_alphanumunderscore, Statement };
use crate::protocol::MessageType;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct BulkInsertStatement {
    #[validate(custom(function = "validate_alphanumunderscore"))]
    #[serde(rename = "table_name")]
    pub table_name: String,

    #[validate(length(min = 1))]
    #[serde(rename = "rows")]
    pub rows: Vec<HashMap<String, serde_json::Value>>,
}

#[allow(dead_code)]
impl BulkInsertStatement {
    pub fn new(
        table_name: String,
        rows: Vec<HashMap<String, serde_json::Value>>,
    ) -> Result<Self, ValidationErrors> {
        let stmt = BulkInsertStatement { table_name, rows };
        stmt.validate()?;
        Ok(stmt)
    }
}

impl Statement for BulkInsertStatement {
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }

    fn protocol(&self) -> MessageType {
        MessageType::BulkInsert
    }

    fn to_bytes(&self) -> Result<Vec<u8>, encode::Error> {
        encode::to_vec(self)
    }

    fn from_bytes(data: &[u8]) -> Result<Box<dyn Statement>, decode::Error> {
        let stmt: BulkInsertStatement = decode::from_slice(data)?;
        Ok(Box::new(stmt))
    }

    fn to_string(&self) -> String {
        format!(
            "BulkInsertStatement{{TableName: {}, Rows: {}}}",
            self.table_name,
            self.rows.len()
        )
    }
}
