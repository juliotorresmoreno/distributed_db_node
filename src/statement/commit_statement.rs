use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};
use rmp_serde::{encode, decode};
use crate::statement::validate::validate_alphanumunderscore;
use crate::protocol::MessageType;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct CommitStatement {
    #[validate(custom(function = "validate_alphanumunderscore"))]
    #[serde(rename = "transaction_id")]
    pub transaction_id: String,
}

impl CommitStatement {
    pub fn new(transaction_id: String) -> Result<Self, ValidationErrors> {
        let stmt = CommitStatement { transaction_id };
        stmt.validate()?;
        Ok(stmt)
    }

    pub fn protocol(&self) -> MessageType {
        MessageType::Commit
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, encode::Error> {
        encode::to_vec(self)
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self, decode::Error> {
        decode::from_slice(data)
    }

    pub fn to_string(&self) -> String {
        format!("CommitStatement{{TransactionID: {}}}", self.transaction_id)
    }
}
