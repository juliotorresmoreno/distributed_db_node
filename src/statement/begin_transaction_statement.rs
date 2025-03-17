use serde::{ Deserialize, Serialize };
use validator::{ Validate, ValidationErrors };
use rmp_serde::{ encode, decode };
use crate::statement::{ validate_alphanumunderscore, Statement };
use crate::protocol::MessageType;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct BeginTransactionStatement {
    #[validate(custom(function = "validate_alphanumunderscore"))]
    #[serde(rename = "transaction_id")]
    pub transaction_id: String,
}

#[allow(dead_code)]
impl BeginTransactionStatement {
    pub fn new(transaction_id: String) -> Result<Self, ValidationErrors> {
        let stmt = BeginTransactionStatement { transaction_id };
        stmt.validate()?;
        Ok(stmt)
    }
}

impl Statement for BeginTransactionStatement {
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }

    fn protocol(&self) -> MessageType {
        MessageType::BeginTransaction
    }

    fn to_bytes(&self) -> Result<Vec<u8>, encode::Error> {
        encode::to_vec(self)
    }

    fn from_bytes(data: &[u8]) -> Result<Box<dyn Statement>, decode::Error> {
        let stmt: BeginTransactionStatement = decode::from_slice(data)?;
        Ok(Box::new(stmt))
    }

    fn to_string(&self) -> String {
        format!("BeginTransactionStatement{{TransactionID: {}}}", self.transaction_id)
    }
}
