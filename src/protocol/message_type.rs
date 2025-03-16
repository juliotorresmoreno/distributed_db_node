use std::collections::HashMap;
use lazy_static::lazy_static;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum MessageType {
    // Database Management
    CreateDatabase = 1,
    DropDatabase = 2,
    ShowDatabases = 3,

    // Table Operations
    CreateTable = 10,
    DropTable = 11,
    AlterTable = 12,
    RenameTable = 13,
    TruncateTable = 14,
    ShowTables = 15,
    DescribeTable = 16,

    // Index Operations
    CreateIndex = 20,
    DropIndex = 21,
    ShowIndexes = 22,

    // Data Operations
    Insert = 30,
    Select = 31,
    Update = 32,
    Delete = 33,
    BulkInsert = 34,
    Upsert = 35,

    // Transaction Management
    BeginTransaction = 40,
    Commit = 41,
    Rollback = 42,
    Savepoint = 43,
    ReleaseSavepoint = 44,

    // Authentication & User Management
    Login = 50,

    // Utility Commands
    Ping = 90,
    Pong = 91,
    Greeting = 92,
    Welcome = 93,
    UnknownCommand = 255,
}

impl MessageType {
    pub fn to_u32(&self) -> u32 {
        *self as u32
    }
    pub fn from_id(id: u32) -> Self {
        match id {
            1 => MessageType::CreateDatabase,
            2 => MessageType::DropDatabase,
            3 => MessageType::ShowDatabases,

            10 => MessageType::CreateTable,
            11 => MessageType::DropTable,
            12 => MessageType::AlterTable,
            13 => MessageType::RenameTable,
            14 => MessageType::TruncateTable,
            15 => MessageType::ShowTables,
            16 => MessageType::DescribeTable,

            20 => MessageType::CreateIndex,
            21 => MessageType::DropIndex,
            22 => MessageType::ShowIndexes,

            30 => MessageType::Insert,
            31 => MessageType::Select,
            32 => MessageType::Update,
            33 => MessageType::Delete,
            34 => MessageType::BulkInsert,
            35 => MessageType::Upsert,

            40 => MessageType::BeginTransaction,
            41 => MessageType::Commit,
            42 => MessageType::Rollback,
            43 => MessageType::Savepoint,
            44 => MessageType::ReleaseSavepoint,

            50 => MessageType::Login,

            90 => MessageType::Ping,
            91 => MessageType::Pong,
            92 => MessageType::Greeting,
            93 => MessageType::Welcome,

            _ => MessageType::UnknownCommand,
        }
    }

    pub fn to_name(&self) -> &'static str {
        match self {
            MessageType::CreateDatabase => "CreateDatabase",
            MessageType::DropDatabase => "DropDatabase",
            MessageType::ShowDatabases => "ShowDatabases",

            MessageType::CreateTable => "CreateTable",
            MessageType::DropTable => "DropTable",
            MessageType::AlterTable => "AlterTable",
            MessageType::RenameTable => "RenameTable",
            MessageType::TruncateTable => "TruncateTable",
            MessageType::ShowTables => "ShowTables",
            MessageType::DescribeTable => "DescribeTable",

            MessageType::CreateIndex => "CreateIndex",
            MessageType::DropIndex => "DropIndex",
            MessageType::ShowIndexes => "ShowIndexes",

            MessageType::Insert => "Insert",
            MessageType::Select => "Select",
            MessageType::Update => "Update",
            MessageType::Delete => "Delete",
            MessageType::BulkInsert => "BulkInsert",
            MessageType::Upsert => "Upsert",

            MessageType::BeginTransaction => "BeginTransaction",
            MessageType::Commit => "Commit",
            MessageType::Rollback => "Rollback",
            MessageType::Savepoint => "Savepoint",
            MessageType::ReleaseSavepoint => "ReleaseSavepoint",

            MessageType::Login => "Login",

            MessageType::Ping => "Ping",
            MessageType::Pong => "Pong",
            MessageType::Greeting => "Greeting",
            MessageType::Welcome => "Welcome",

            MessageType::UnknownCommand => "UnknownCommand",
        }
    }
}

lazy_static! {
    static ref MESSAGE_TYPE_LOOKUP: HashMap<&'static str, MessageType> = {
        let mut map = HashMap::new();
        map.insert("CreateDatabase", MessageType::CreateDatabase);
        map.insert("DropDatabase", MessageType::DropDatabase);
        map.insert("ShowDatabases", MessageType::ShowDatabases);

        map.insert("CreateTable", MessageType::CreateTable);
        map.insert("DropTable", MessageType::DropTable);
        map.insert("AlterTable", MessageType::AlterTable);
        map.insert("RenameTable", MessageType::RenameTable);
        map.insert("TruncateTable", MessageType::TruncateTable);
        map.insert("ShowTables", MessageType::ShowTables);
        map.insert("DescribeTable", MessageType::DescribeTable);

        map.insert("CreateIndex", MessageType::CreateIndex);
        map.insert("DropIndex", MessageType::DropIndex);
        map.insert("ShowIndexes", MessageType::ShowIndexes);

        map.insert("Insert", MessageType::Insert);
        map.insert("Select", MessageType::Select);
        map.insert("Update", MessageType::Update);
        map.insert("Delete", MessageType::Delete);
        map.insert("BulkInsert", MessageType::BulkInsert);
        map.insert("Upsert", MessageType::Upsert);

        map.insert("BeginTransaction", MessageType::BeginTransaction);
        map.insert("Commit", MessageType::Commit);
        map.insert("Rollback", MessageType::Rollback);
        map.insert("Savepoint", MessageType::Savepoint);
        map.insert("ReleaseSavepoint", MessageType::ReleaseSavepoint);

        map.insert("Login", MessageType::Login);

        map.insert("Ping", MessageType::Ping);
        map.insert("Pong", MessageType::Pong);
        map.insert("Greeting", MessageType::Greeting);
        map.insert("Welcome", MessageType::Welcome);

        map.insert("UnknownCommand", MessageType::UnknownCommand);
        map
    };
}

impl MessageType {
    pub fn from_name(name: &str) -> Self {
        MESSAGE_TYPE_LOOKUP.get(name).cloned().unwrap_or(MessageType::UnknownCommand)
    }
}
