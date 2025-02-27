
type MessageType = u32;

pub const MESSAGE_TYPE_PING: MessageType = 1;
pub const MESSAGE_TYPE_CREATE_TABLE: MessageType = 2;
pub const MESSAGE_TYPE_DROP_TABLE: MessageType = 3;
pub const MESSAGE_TYPE_ALTER_TABLE: MessageType = 4;
pub const MESSAGE_TYPE_CREATE_INDEX: MessageType = 5;
pub const MESSAGE_TYPE_DROP_INDEX: MessageType = 6;
pub const MESSAGE_TYPE_INSERT: MessageType = 7;
pub const MESSAGE_TYPE_SELECT: MessageType = 8;
pub const MESSAGE_TYPE_UPDATE: MessageType = 9;
pub const MESSAGE_TYPE_DELETE: MessageType = 10;
pub const MESSAGE_TYPE_BEGIN_TRANSACTION: MessageType = 11;
pub const MESSAGE_TYPE_COMMIT: MessageType = 12;
pub const MESSAGE_TYPE_ROLLBACK: MessageType = 13;
pub const MESSAGE_TYPE_PONG: MessageType = 14;
pub const MESSAGE_TYPE_GREETING: MessageType = 15;
pub const MESSAGE_TYPE_WELCOME: MessageType = 16;
pub const MESSAGE_TYPE_UNKNOWN_COMMAND: MessageType = 255;

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
