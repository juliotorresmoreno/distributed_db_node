use crate::protocol::MessageType;
use crate::statement::*;
use crate::statement::error::UnsupportedStatementError;

pub trait Statement {
    fn clone_box(&self) -> Box<dyn Statement>;
    fn protocol(&self) -> MessageType;
    fn to_bytes(&self) -> Result<Vec<u8>, rmp_serde::encode::Error>;

    fn from_bytes(data: &[u8]) -> Result<Box<dyn Statement>, rmp_serde::decode::Error>
        where Self: Sized;

    fn to_string(&self) -> String;
}

impl Clone for Box<dyn Statement> {
    fn clone(&self) -> Box<dyn Statement> {
        self.clone_box()
    }
}

pub fn deserialize_statement(
    message_type: MessageType,
    data: &[u8]
) -> Result<Box<dyn Statement>, UnsupportedStatementError> {
    let result = match message_type {
        // Database Management
        MessageType::CreateDatabase =>
            CreateDatabaseStatement::from_bytes(data).map_err(|_| UnsupportedStatementError {
                message_type: MessageType::CreateDatabase,
                message: "Unsupported statement".to_string(),
            }),
        MessageType::DropDatabase =>
            DropDatabaseStatement::from_bytes(data).map_err(|_| UnsupportedStatementError {
                message_type: MessageType::DropDatabase,
                message: "Unsupported statement".to_string(),
            }),

        // Table Operations
        MessageType::CreateTable =>
            CreateTableStatement::from_bytes(data).map_err(|_| UnsupportedStatementError {
                message_type: MessageType::CreateTable,
                message: "Unsupported statement".to_string(),
            }),
        MessageType::DropTable =>
            DropTableStatement::from_bytes(data).map_err(|_| UnsupportedStatementError {
                message_type: MessageType::DropTable,
                message: "Unsupported statement".to_string(),
            }),
        MessageType::AlterTable =>
            AlterTableStatement::from_bytes(data).map_err(|_| UnsupportedStatementError {
                message_type: MessageType::AlterTable,
                message: "Unsupported statement".to_string(),
            }),
        MessageType::RenameTable =>
            RenameTableStatement::from_bytes(data).map_err(|_| UnsupportedStatementError {
                message_type: MessageType::RenameTable,
                message: "Unsupported statement".to_string(),
            }),
        MessageType::TruncateTable =>
            TruncateTableStatement::from_bytes(data).map_err(|_| UnsupportedStatementError {
                message_type: MessageType::TruncateTable,
                message: "Unsupported statement".to_string(),
            }),
        MessageType::ShowTables =>
            EmptyStatement::from_bytes(data).map_err(|_| UnsupportedStatementError {
                message_type: MessageType::ShowTables,
                message: "Unsupported statement".to_string(),
            }),
        MessageType::DescribeTable =>
            DescribeTableStatement::from_bytes(data).map_err(|_| UnsupportedStatementError {
                message_type: MessageType::DescribeTable,
                message: "Unsupported statement".to_string(),
            }),

        // Index Operations
        MessageType::CreateIndex =>
            CreateIndexStatement::from_bytes(data).map_err(|_| UnsupportedStatementError {
                message_type: MessageType::CreateIndex,
                message: "Unsupported statement".to_string(),
            }),
        MessageType::DropIndex =>
            DropIndexStatement::from_bytes(data).map_err(|_| UnsupportedStatementError {
                message_type: MessageType::DropIndex,
                message: "Unsupported statement".to_string(),
            }),
        MessageType::ShowIndexes =>
            ShowIndexesStatement::from_bytes(data).map_err(|_| UnsupportedStatementError {
                message_type: MessageType::ShowIndexes,
                message: "Unsupported statement".to_string(),
            }),

        // Data Operations
        MessageType::Insert =>
            InsertStatement::from_bytes(data).map_err(|_| UnsupportedStatementError {
                message_type: MessageType::Insert,
                message: "Unsupported statement".to_string(),
            }),
        MessageType::Select =>
            SelectStatement::from_bytes(data).map_err(|_| UnsupportedStatementError {
                message_type: MessageType::Select,
                message: "Unsupported statement".to_string(),
            }),
        MessageType::Update =>
            UpdateStatement::from_bytes(data).map_err(|_| UnsupportedStatementError {
                message_type: MessageType::Update,
                message: "Unsupported statement".to_string(),
            }),
        MessageType::Delete =>
            DeleteStatement::from_bytes(data).map_err(|_| UnsupportedStatementError {
                message_type: MessageType::Delete,
                message: "Unsupported statement".to_string(),
            }),
        MessageType::BulkInsert =>
            BulkInsertStatement::from_bytes(data).map_err(|_| UnsupportedStatementError {
                message_type: MessageType::BulkInsert,
                message: "Unsupported statement".to_string(),
            }),
        MessageType::Upsert =>
            UpsertStatement::from_bytes(data).map_err(|_| UnsupportedStatementError {
                message_type: MessageType::Upsert,
                message: "Unsupported statement".to_string(),
            }),

        // Transaction Management
        MessageType::BeginTransaction =>
            BeginTransactionStatement::from_bytes(data).map_err(|_| UnsupportedStatementError {
                message_type: MessageType::BeginTransaction,
                message: "Unsupported statement".to_string(),
            }),
        MessageType::Commit =>
            CommitStatement::from_bytes(data).map_err(|_| UnsupportedStatementError {
                message_type: MessageType::Commit,
                message: "Unsupported statement".to_string(),
            }),
        MessageType::Rollback =>
            RollbackStatement::from_bytes(data).map_err(|_| UnsupportedStatementError {
                message_type: MessageType::Rollback,
                message: "Unsupported statement".to_string(),
            }),
        MessageType::Savepoint =>
            SavepointStatement::from_bytes(data).map_err(|_| UnsupportedStatementError {
                message_type: MessageType::Savepoint,
                message: "Unsupported statement".to_string(),
            }),
        MessageType::ReleaseSavepoint =>
            ReleaseSavepointStatement::from_bytes(data).map_err(|_| UnsupportedStatementError {
                message_type: MessageType::ReleaseSavepoint,
                message: "Unsupported statement".to_string(),
            }),

        // Authentication & User Management
        MessageType::Login =>
            LoginStatement::from_bytes(data).map_err(|_| UnsupportedStatementError {
                message_type: MessageType::Login,
                message: "Unsupported statement".to_string(),
            }),

        // Utility Commands
        MessageType::Ping =>
            EmptyStatement::from_bytes(data).map_err(|_| UnsupportedStatementError {
                message_type: MessageType::Ping,
                message: "Unsupported statement".to_string(),
            }),
        MessageType::Pong =>
            EmptyStatement::from_bytes(data).map_err(|_| UnsupportedStatementError {
                message_type: MessageType::Pong,
                message: "Unsupported statement".to_string(),
            }),
        MessageType::Greeting =>
            EmptyStatement::from_bytes(data).map_err(|_| UnsupportedStatementError {
                message_type: MessageType::Greeting,
                message: "Unsupported statement".to_string(),
            }),
        MessageType::Welcome =>
            EmptyStatement::from_bytes(data).map_err(|_| UnsupportedStatementError {
                message_type: MessageType::Welcome,
                message: "Unsupported statement".to_string(),
            }),

        // Unsupported
        _ => Err(UnsupportedStatementError {
            message_type: message_type,
            message: "Unsupported statement".to_string(),
        }),
    };

    result
}
