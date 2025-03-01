use std::error::Error;
use serde::{ Serialize, Deserialize };

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ColumnDefinition {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "type")]
    pub r#type: String,

    #[serde(rename = "length")]
    pub length: i32,

    #[serde(rename = "primary_key")]
    pub primary_key: bool,

    #[serde(rename = "index")]
    pub index: bool,

    #[serde(rename = "default_value")]
    pub default_value: String,
}

#[derive(Debug, Clone, Copy)]
pub enum MessageType {
    // =====================
    // Database Management
    // =====================
    CreateDatabase,
    DropDatabase,
    ShowDatabases,
    UseDatabase,

    // =====================
    // Table Operations
    // =====================
    CreateTable,
    DropTable,
    AlterTable,
    RenameTable,
    TruncateTable,
    ShowTables,
    DescribeTable,

    // =====================
    // Index Operations
    // =====================
    CreateIndex,
    DropIndex,
    ShowIndexes,

    // =====================
    // Data Operations
    // =====================
    Insert,
    Select,
    Update,
    Delete,
    BulkInsert,
    Upsert,

    // =====================
    // Transaction Management
    // =====================
    BeginTransaction,
    Commit,
    Rollback,
    #[allow(dead_code)]
    Savepoint,
    #[allow(dead_code)]
    ReleaseSavepoint,

    // =====================
    // Utility Commands
    // =====================
    #[allow(dead_code)]
    Ping,
    #[allow(dead_code)]
    Pong,
    #[allow(dead_code)]
    Greeting,
    #[allow(dead_code)]
    Welcome,
    #[allow(dead_code)]
    UnknownCommand,
}

pub trait Statement {
    fn protocol(&self) -> MessageType;
    fn to_bytes(&self) -> Result<Vec<u8>, Box<dyn Error>>;
    fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn Error>> where Self: Sized;
}
