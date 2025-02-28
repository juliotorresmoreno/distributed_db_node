// Define the base type for MessageType
pub type MessageType = u32;

// Database Management
pub const MESSAGE_TYPE_CREATE_DATABASE: MessageType = 1;
pub const MESSAGE_TYPE_DROP_DATABASE: MessageType = 2;
pub const MESSAGE_TYPE_SHOW_DATABASES: MessageType = 3;

// Table Operations
pub const MESSAGE_TYPE_CREATE_TABLE: MessageType = 10;
pub const MESSAGE_TYPE_DROP_TABLE: MessageType = 11;
pub const MESSAGE_TYPE_ALTER_TABLE: MessageType = 12;
pub const MESSAGE_TYPE_RENAME_TABLE: MessageType = 13;
pub const MESSAGE_TYPE_TRUNCATE_TABLE: MessageType = 14;
pub const MESSAGE_TYPE_SHOW_TABLES: MessageType = 15;
pub const MESSAGE_TYPE_DESCRIBE_TABLE: MessageType = 16;

// Index Operations
pub const MESSAGE_TYPE_CREATE_INDEX: MessageType = 20;
pub const MESSAGE_TYPE_DROP_INDEX: MessageType = 21;
pub const MESSAGE_TYPE_SHOW_INDEXES: MessageType = 22;

// Data Operations
pub const MESSAGE_TYPE_INSERT: MessageType = 30;
pub const MESSAGE_TYPE_SELECT: MessageType = 31;
pub const MESSAGE_TYPE_UPDATE: MessageType = 32;
pub const MESSAGE_TYPE_DELETE: MessageType = 33;
pub const MESSAGE_TYPE_BULK_INSERT: MessageType = 34;
pub const MESSAGE_TYPE_UPSERT: MessageType = 35;

// Transaction Management
pub const MESSAGE_TYPE_BEGIN_TRANSACTION: MessageType = 40;
pub const MESSAGE_TYPE_COMMIT: MessageType = 41;
pub const MESSAGE_TYPE_ROLLBACK: MessageType = 42;
pub const MESSAGE_TYPE_SAVEPOINT: MessageType = 43;
pub const MESSAGE_TYPE_RELEASE_SAVEPOINT: MessageType = 44;

// Utility Commands
pub const MESSAGE_TYPE_PING: MessageType = 90;
pub const MESSAGE_TYPE_PONG: MessageType = 91;
pub const MESSAGE_TYPE_GREETING: MessageType = 92;
pub const MESSAGE_TYPE_WELCOME: MessageType = 93;
pub const MESSAGE_TYPE_UNKNOWN_COMMAND: MessageType = 255;

// Mapping from MessageType to their String representations
pub fn get_message_type_name(message_type: MessageType) -> &'static str {
    match message_type {
        // Database Management
        MESSAGE_TYPE_CREATE_DATABASE => "CreateDatabase",
        MESSAGE_TYPE_DROP_DATABASE => "DropDatabase",
        MESSAGE_TYPE_SHOW_DATABASES => "ShowDatabases",

        // Table Operations
        MESSAGE_TYPE_CREATE_TABLE => "CreateTable",
        MESSAGE_TYPE_DROP_TABLE => "DropTable",
        MESSAGE_TYPE_ALTER_TABLE => "AlterTable",
        MESSAGE_TYPE_RENAME_TABLE => "RenameTable",
        MESSAGE_TYPE_TRUNCATE_TABLE => "TruncateTable",
        MESSAGE_TYPE_SHOW_TABLES => "ShowTables",
        MESSAGE_TYPE_DESCRIBE_TABLE => "DescribeTable",

        // Index Operations
        MESSAGE_TYPE_CREATE_INDEX => "CreateIndex",
        MESSAGE_TYPE_DROP_INDEX => "DropIndex",
        MESSAGE_TYPE_SHOW_INDEXES => "ShowIndexes",

        // Data Operations
        MESSAGE_TYPE_INSERT => "Insert",
        MESSAGE_TYPE_SELECT => "Select",
        MESSAGE_TYPE_UPDATE => "Update",
        MESSAGE_TYPE_DELETE => "Delete",
        MESSAGE_TYPE_BULK_INSERT => "BulkInsert",
        MESSAGE_TYPE_UPSERT => "Upsert",

        // Transaction Management
        MESSAGE_TYPE_BEGIN_TRANSACTION => "BeginTransaction",
        MESSAGE_TYPE_COMMIT => "Commit",
        MESSAGE_TYPE_ROLLBACK => "Rollback",
        MESSAGE_TYPE_SAVEPOINT => "Savepoint",
        MESSAGE_TYPE_RELEASE_SAVEPOINT => "ReleaseSavepoint",

        // Utility Commands
        MESSAGE_TYPE_PING => "Ping",
        MESSAGE_TYPE_PONG => "Pong",
        MESSAGE_TYPE_GREETING => "Greeting",
        MESSAGE_TYPE_WELCOME => "Welcome",
        MESSAGE_TYPE_UNKNOWN_COMMAND => "UnknownCommand",

        // Default case for unknown message types
        _ => "UnknownMessageType",
    }
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
