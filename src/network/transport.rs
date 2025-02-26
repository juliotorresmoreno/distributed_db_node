use serde::{Deserialize, Serialize};

#[derive(Debug)]
struct MessageHeader {
    body_size: u32,
}

impl MessageHeader {
    fn to_bytes(&self) -> [u8; 4] {
        self.body_size.to_be_bytes()
    }

    fn from_bytes(bytes: [u8; 4]) -> Self {
        let body_size = u32::from_be_bytes(bytes);
        Self { body_size }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct MessageBody {
    key: String,
    value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    message: Option<String>,
}