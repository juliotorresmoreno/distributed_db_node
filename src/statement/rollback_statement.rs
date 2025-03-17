use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};
use rmp_serde::{encode, decode};
use crate::protocol::MessageType;
use crate::statement::{ validate_alphanumunderscore, Statement };

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct RollbackStatement {
    #[validate(custom(function = "validate_alphanumunderscore"))]
    #[serde(rename = "transaction_id")]
    pub transaction_id: String,
}

#[allow(dead_code)]
impl RollbackStatement {
    pub fn new(transaction_id: String) -> Result<Self, ValidationErrors> {
        let stmt = RollbackStatement { transaction_id };
        stmt.validate()?;
        Ok(stmt)
    }
}

impl Statement for RollbackStatement {
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }

    fn protocol(&self) -> MessageType {
        MessageType::Rollback
    }

    fn to_bytes(&self) -> Result<Vec<u8>, encode::Error> {
        encode::to_vec(self)
    }

    fn from_bytes(data: &[u8]) -> Result<Box<dyn Statement>, decode::Error> {
        let stmt: RollbackStatement = decode::from_slice(data)?;
        Ok(Box::new(stmt))
    }

    fn to_string(&self) -> String {
        format!("RollbackStatement{{TransactionID: {}}}", self.transaction_id)
    }
}
