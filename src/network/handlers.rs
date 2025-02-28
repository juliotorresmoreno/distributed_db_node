use super::transport::*;
use crate::{
    network::server::Server,
    protocol::{ createDatabase::CreateDatabaseStatement, statement::Statement },
};
use log::{ info, error };
use crate::protocol::createTable::CreateTableStatement;

// =====================
// Database Management
// =====================

pub async fn handle_create_database(server: &mut Server, message: &Message) {
    info!("Received CREATE TABLE");

    let stm = match CreateDatabaseStatement::from_bytes(&message.body) {
        Ok(statement) => statement,
        Err(e) => {
            error!("Failed to parse BSON for CREATE TABLE: {}", e);
            return;
        }
    };

    server.storage().lock().unwrap().create_database(&stm.database_name);

    let body = b"DATABASE CREATED";
    match server.send(message.header.message_id, MESSAGE_TYPE_CREATE_TABLE, body).await {
        Ok(_) => info!("Database created"),
        Err(e) => error!("Failed to send response: {}", e),
    }
}

pub async fn handle_drop_database(server: &mut Server, message: &Message) {
    info!("Received DROP DATABASE");
}

pub async fn handle_show_databases(server: &mut Server, message: &Message) {
    info!("Received SHOW DATABASES");
}

// =====================
// Table Operations
// =====================

pub async fn handle_create_table(server: &mut Server, message: &Message) {
    info!("Received CREATE TABLE");

    let stm = match CreateTableStatement::from_bytes(&message.body) {
        Ok(statement) => statement,
        Err(e) => {
            error!("Failed to parse BSON for CREATE TABLE: {}", e);
            return;
        }
    };

    server.storage().lock().unwrap().create_table("", &stm.table_name, &stm.columns, &stm.storage);

    let body = b"TABLE CREATED";
    match server.send(message.header.message_id, MESSAGE_TYPE_CREATE_TABLE, body).await {
        Ok(_) => info!("Table created"),
        Err(e) => error!("Failed to send response: {}", e),
    }
}

pub async fn handle_drop_table(server: &mut Server, message: &Message) {
    info!("Received DROP TABLE");
}

pub async fn handle_alter_table(server: &mut Server, message: &Message) {
    info!("Received ALTER TABLE");
}

pub async fn handle_rename_table(server: &mut Server, message: &Message) {
    info!("Received RENAME TABLE");
}

pub async fn handle_truncate_table(server: &mut Server, message: &Message) {
    info!("Received TRUNCATE TABLE");
}

pub async fn handle_show_tables(server: &mut Server, message: &Message) {
    info!("Received SHOW TABLES");
}

pub async fn handle_describe_table(server: &mut Server, message: &Message) {
    info!("Received DESCRIBE TABLE");
}

// =====================
// Index Operations
// =====================

pub async fn handle_create_index(server: &mut Server, message: &Message) {
    info!("Received CREATE INDEX");
}

pub async fn handle_drop_index(server: &mut Server, message: &Message) {
    info!("Received DROP INDEX");
}

pub async fn handle_show_indexes(server: &mut Server, message: &Message) {
    info!("Received SHOW INDEXES");
}

// =====================
// Data Operations
// =====================

pub async fn handle_insert(server: &mut Server, message: &Message) {
    info!("Received INSERT");
}

pub async fn handle_select(server: &mut Server, message: &Message) {
    info!("Received SELECT");
}

pub async fn handle_update(server: &mut Server, message: &Message) {
    info!("Received UPDATE");
}

pub async fn handle_delete(server: &mut Server, message: &Message) {
    info!("Received DELETE");
}

pub async fn handle_bulk_insert(server: &mut Server, message: &Message) {
    info!("Received BULK INSERT");
}

pub async fn handle_upsert(server: &mut Server, message: &Message) {
    info!("Received UPSERT");
}

// =====================
// Transaction Management
// =====================

pub async fn handle_begin_transaction(server: &mut Server, message: &Message) {
    info!("Received BEGIN TRANSACTION");
}

pub async fn handle_commit(server: &mut Server, message: &Message) {
    info!("Received COMMIT");
}

pub async fn handle_rollback(server: &mut Server, message: &Message) {
    info!("Received ROLLBACK");
}

pub async fn handle_savepoint(server: &mut Server, message: &Message) {
    info!("Received SAVEPOINT");
}

pub async fn handle_release_savepoint(server: &mut Server, message: &Message) {
    info!("Received RELEASE SAVEPOINT");
}

// =====================
// Utility Commands
// =====================

pub async fn handle_ping(server: &mut Server, message: &Message) {
    info!("Received PING");
    info!("Sending PONG");

    let body = b"PONG";
    match server.send(message.header.message_id, MESSAGE_TYPE_PONG, body).await {
        Ok(_) => {
            info!("PONG sent");
        }
        Err(e) => {
            error!("Failed to send PONG: {}", e);
        }
    }
}

pub async fn handle_pong(server: &mut Server, message: &Message) {
    info!("Received PONG");
}

pub async fn handle_greeting(server: &mut Server, message: &Message) {
    info!("Received GREETING");
}

pub async fn handle_welcome(server: &mut Server, message: &Message) {
    info!("Received WELCOME");
}

pub async fn handle_unknown_command(server: &mut Server, message: &Message) {
    error!("Received UNKNOWN COMMAND");
}
