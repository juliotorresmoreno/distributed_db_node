use serde::{ Deserialize, Serialize };
use validator::{ Validate, ValidationErrors };
use rmp_serde::{ encode, decode };
use crate::protocol::MessageType;
use crate::statement::validate::validate_alphanumunderscore;
use crate::statement::ColumnDefinition;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct CreateTableStatement {
    #[validate(custom(function = "validate_alphanumunderscore"))]
    #[serde(rename = "table_name")]
    pub table_name: String,

    #[validate(length(min = 1))]
    #[serde(rename = "columns")]
    pub columns: Vec<ColumnDefinition>,

    #[serde(rename = "storage")]
    pub storage: Option<String>,
}

impl CreateTableStatement {
    pub fn new(
        table_name: String,
        columns: Vec<ColumnDefinition>,
        storage: Option<String>
    ) -> Result<Self, ValidationErrors> {
        let stmt = CreateTableStatement { table_name, columns, storage };
        stmt.validate()?;
        Ok(stmt)
    }

    pub fn protocol(&self) -> MessageType {
        MessageType::CreateTable
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, encode::Error> {
        encode::to_vec(self)
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self, decode::Error> {
        decode::from_slice(data)
    }

    pub fn to_string(&self) -> String {
        format!(
            "CreateTableStatement{{TableName: {}, Columns: {:?}, Storage: {:?}}}",
            self.table_name,
            self.columns,
            self.storage
        )
    }
}
