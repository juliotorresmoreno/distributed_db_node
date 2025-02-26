#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageType {
    Ping = 1,
    CreateTable = 2,
    DropTable = 3,
    AlterTable = 4,
    CreateIndex = 5,
    DropIndex = 6,
    Insert = 7,
    Select = 8,
    Update = 9,
    Delete = 10,
    BeginTransaction = 11,
    Commit = 12,
    Rollback = 13,
    UnknownCommand = 255,
}

#[derive(Debug)]
pub struct MessageHeader {
    pub message_id: [u8; 16],
    pub message_type: u32,
    pub body_size: u32,
}

pub struct Message {
    pub header: MessageHeader,
    pub body: Vec<u8>,
}

impl Message {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.header.to_bytes());
        bytes.extend_from_slice(&self.body);

        return bytes;
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        let header_bytes = bytes[..24].try_into().unwrap();
        let header = MessageHeader::from_bytes(header_bytes);
        let body = bytes[24..].to_vec();

        return Self { header, body };
    }
}

impl MessageHeader {
    pub fn to_bytes(&self) -> [u8; 24] {
        let mut bytes = [0; 24];

        bytes[..16].copy_from_slice(&self.message_id);
        bytes[16..20].copy_from_slice(&self.message_type.to_be_bytes());
        bytes[20..24].copy_from_slice(&self.body_size.to_be_bytes());

        return bytes;
    }

    pub fn from_bytes(bytes: [u8; 24]) -> Self {
        let message_id = bytes[..16].try_into().unwrap();
        let message_type = u32::from_be_bytes(bytes[16..20].try_into().unwrap());
        let body_size = u32::from_be_bytes(bytes[20..24].try_into().unwrap());

        return Self {
            message_id,
            message_type,
            body_size,
        };
    }
}
