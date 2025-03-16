use std::fmt;

use crate::protocol::MessageType;

#[derive(Debug)]
pub struct UnsupportedStatementError {
    pub message_type: MessageType,
    pub message: String,
}

impl fmt::Display for UnsupportedStatementError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unsupported statement type")
    }
}

impl UnsupportedStatementError {
    pub fn new(message_type: MessageType, message: String) -> Self {
        return Self {
            message_type: message_type,
            message: message,
        }
    }
}
