use serde::{ Deserialize, Serialize };
use validator::{ Validate, ValidationErrors };
use rmp_serde::{ encode, decode };
use crate::protocol::MessageType;
use crate::utils;
use regex::Regex;
use subtle::ConstantTimeEq;
use chrono::Utc;

use super::Statement;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct LoginStatement {
    #[serde(rename = "timestamp")]
    pub timestamp: u64,

    #[serde(rename = "is_replica")]
    pub is_replica: bool,

    #[serde(rename = "hash")]
    pub hash: String,

    #[serde(rename = "node_name")]
    pub node_name: String,

    #[serde(rename = "node_id")]
    pub node_id: String,

    #[serde(rename = "tags")]
    pub tags: Vec<String>,
}

impl LoginStatement {
    pub fn new(
        token: &str,
        node_id: String,
        node_name: String,
        is_replica: bool,
        tags: Vec<String>
    ) -> Result<Self, ValidationErrors> {
        if token.is_empty() {
            return Err(ValidationErrors::new());
        }

        let re_node_id = Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();
        let re_node_name = Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();

        if node_id.is_empty() || !re_node_id.is_match(&node_id) {
            return Err(ValidationErrors::new());
        }

        if node_name.is_empty() || !re_node_name.is_match(&node_name) {
            return Err(ValidationErrors::new());
        }

        if tags.is_empty() || tags.iter().any(|t| (t.is_empty() || !re_node_id.is_match(t))) {
            return Err(ValidationErrors::new());
        }

        let timestamp = Utc::now().timestamp_nanos() as u64;
        let hash = utils::generate_hash(token, timestamp, &node_id, is_replica, &tags);

        let stmt = LoginStatement {
            timestamp,
            is_replica,
            hash,
            node_name,
            node_id,
            tags,
        };

        stmt.validate()?;
        Ok(stmt)
    }

    fn validate_hash(&self, token: &str) -> bool {
        let expected = utils::generate_hash(
            token,
            self.timestamp,
            &self.node_id,
            self.is_replica,
            &self.tags
        );
        self.hash.as_bytes().ct_eq(expected.as_bytes()).unwrap_u8() == 1
    }
}

impl Statement for LoginStatement {
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }

    fn protocol(&self) -> MessageType {
        MessageType::Login
    }

    fn to_bytes(&self) -> Result<Vec<u8>, encode::Error> {
        encode::to_vec(self)
    }

    fn from_bytes(data: &[u8]) -> Result<Box<dyn Statement>, decode::Error> {
        let stmt: LoginStatement = decode::from_slice(data)?;
        Ok(Box::new(stmt))
    }

    fn to_string(&self) -> String {
        format!(
            "LoginStatement{{Timestamp: {}, NodeID: {}, NodeName: {}, IsReplica: {}, Tags: {:?}}}",
            self.timestamp,
            self.node_id,
            self.node_name,
            self.is_replica,
            self.tags
        )
    }
}
