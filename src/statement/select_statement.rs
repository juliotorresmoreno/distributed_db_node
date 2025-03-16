use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};
use rmp_serde::{encode, decode};
use crate::protocol::MessageType;
use crate::statement::{validate::validate_alphanumunderscore, Statement};

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct SelectStatement {
    #[validate(custom(function = "validate_alphanumunderscore"))]
    #[serde(rename = "table_name")]
    pub table_name: String,

    #[serde(rename = "columns")]
    pub columns: Vec<String>,

    #[serde(rename = "where")]
    pub r#where: String,
}

impl SelectStatement {
    pub fn new(table_name: String, columns: Vec<String>, r#where: String) -> Result<Self, ValidationErrors> {
        let stmt = SelectStatement { table_name, columns, r#where };
        stmt.validate()?;
        Ok(stmt)
    }
}

impl Statement for SelectStatement {
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }

    fn protocol(&self) -> MessageType {
        MessageType::Select
    }

    fn to_bytes(&self) -> Result<Vec<u8>, encode::Error> {
        encode::to_vec(self)
    }

    fn from_bytes(data: &[u8]) -> Result<Box<dyn Statement>, decode::Error> {
        let stmt: SelectStatement = decode::from_slice(data)?;
        Ok(Box::new(stmt))
    }

    fn to_string(&self) -> String {
        format!("SelectStatement{{TableName: {}, Columns: {:?}, Where: {}}}", self.table_name, self.columns, self.r#where)
    }
}
