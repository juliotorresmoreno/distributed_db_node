use serde::{ Deserialize, Serialize };
use validator::{ Validate, ValidationErrors };
use rmp_serde::{ encode, decode };
use crate::protocol::MessageType;
use crate::utils;
use regex::Regex;
use subtle::ConstantTimeEq;

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

        let node_id_re = Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();
        let node_name_re = Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();
        if node_id.is_empty() || !node_id_re.is_match(&node_id) {
            return Err(ValidationErrors::new());
        }
        if node_name.is_empty() || !node_name_re.is_match(&node_name) {
            return Err(ValidationErrors::new());
        }
        if tags.is_empty() || tags.iter().any(|t| (t.is_empty() || !node_id_re.is_match(t))) {
            return Err(ValidationErrors::new());
        }

        let timestamp = chrono::Utc::now().timestamp_nanos_opt().unwrap() as u64;
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

    pub fn validate_hash(&self, token: &str) -> bool {
        let expected = utils::generate_hash(
            token,
            self.timestamp,
            &self.node_id,
            self.is_replica,
            &self.tags
        );
        self.hash.as_bytes().ct_eq(expected.as_bytes()).unwrap_u8() == 1
    }

    pub fn protocol(&self) -> MessageType {
        MessageType::Login
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, encode::Error> {
        encode::to_vec(self)
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self, decode::Error> {
        decode::from_slice(data)
    }

    pub fn to_string(&self) -> String {
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
