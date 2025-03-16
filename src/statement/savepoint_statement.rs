use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};
use rmp_serde::{encode, decode};
use crate::protocol::MessageType;
use crate::statement::{validate::validate_alphanumunderscore, Statement};

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct SavepointStatement {
    #[validate(custom(function = "validate_alphanumunderscore"))]
    #[serde(rename = "transaction_id")]
    pub transaction_id: String,

    #[validate(custom(function = "validate_alphanumunderscore"))]
    #[serde(rename = "savepoint_name")]
    pub savepoint_name: String,
}

impl SavepointStatement {
    pub fn new(transaction_id: String, savepoint_name: String) -> Result<Self, ValidationErrors> {
        let stmt = SavepointStatement { transaction_id, savepoint_name };
        stmt.validate()?;
        Ok(stmt)
    }
}

impl Statement for SavepointStatement {
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }

    fn protocol(&self) -> MessageType {
        MessageType::Savepoint
    }

    fn to_bytes(&self) -> Result<Vec<u8>, encode::Error> {
        encode::to_vec(self)
    }

    fn from_bytes(data: &[u8]) -> Result<Box<dyn Statement>, decode::Error> {
        let stmt: SavepointStatement = decode::from_slice(data)?;
        Ok(Box::new(stmt))
    }

    fn to_string(&self) -> String {
        format!(
            "SavepointStatement{{TransactionID: {}, SavepointName: {}}}",
            self.transaction_id, self.savepoint_name
        )
    }
}
