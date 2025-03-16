use async_trait::async_trait;
use validator::ValidationError;
use crate::protocol::MessageType;
use regex::Regex;


#[async_trait]
pub trait Statement: Send + Sync {
    fn protocol(&self) -> MessageType;
    fn to_bytes(&self) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>>;
    fn from_bytes(&mut self, data: &[u8]) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    fn to_string(&self) -> String;
    fn validate(&self) -> Result<(), String>;
    fn clone_box(&self) -> Box<dyn Statement>;
}

// Necesario para habilitar `.clone_box()` en objetos dinámicos
impl Clone for Box<dyn Statement> {
    fn clone(&self) -> Box<dyn Statement> {
        self.clone_box()
    }
}
