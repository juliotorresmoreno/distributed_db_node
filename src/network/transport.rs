#[derive(Debug)]
pub struct MessageHeader {
    pub message_id: [u8; 16], // 16-byte message ID (e.g., UUID)
    pub body_size: u32,       // 4-byte body size
}

impl MessageHeader {
    /// Serializes the header to a byte array.
    pub fn to_bytes(&self) -> [u8; 20] {
        let mut bytes = [0; 20];
        bytes[..16].copy_from_slice(&self.message_id);
        bytes[16..20].copy_from_slice(&self.body_size.to_be_bytes()); 
        bytes
    }

    pub fn from_bytes(bytes: [u8; 20]) -> Self {
        let message_id = bytes[..16].try_into().unwrap(); 
        let body_size = u32::from_be_bytes(bytes[16..20].try_into().unwrap());
        Self {
            message_id,
            body_size,
        }
    }
}