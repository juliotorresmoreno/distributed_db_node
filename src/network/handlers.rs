use super::transport::*;
use crate::{
    network::server::Server,
    protocol::{
        data::{ bulk_insert::BulkInsertStatement, delete::DeleteStatement, insert::InsertStatement, select::SelectStatement, update::UpdateStatement },
        index::{
            create_index::CreateIndexStatement,
            drop_index::DropIndexStatement,
            show_indexes::ShowIndexesStatement,
        },
        management::{
            create_database::CreateDatabaseStatement,
            drop_database::DropDatabaseStatement,
            use_database::UseDatabaseStatement,
        },
        operations::{
            alter_table::AlterTableStatement,
            create_table::CreateTableStatement,
            describe_table::DescribeTableStatement,
            drop_table::DropTableStatement,
            rename_table::RenameTableStatement,
            truncate_table::TruncateTableStatement,
        },
        statement::Statement, transaction::begin_transaction::BeginTransactionStatement,
    },
};
use log::{ info, error };

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

    let stm = match DropDatabaseStatement::from_bytes(&message.body) {
        Ok(statement) => statement,
        Err(e) => {
            error!("Failed to parse BSON for DROP DATABASE: {}", e);
            return;
        }
    };

    server.storage().lock().unwrap().drop_database(&stm.database_name);

    let body = b"DATABASE DROPPED";
    match server.send(message.header.message_id, MESSAGE_TYPE_DROP_DATABASE, body).await {
        Ok(_) => info!("Database dropped"),
        Err(e) => error!("Failed to send response: {}", e),
    }
}

pub async fn handle_show_databases(server: &mut Server, message: &Message) {
    info!("Received SHOW DATABASES");

    let databases = server.storage().lock().unwrap().show_databases();

    let databases_str = databases.join("\n");
    let body = databases_str.as_bytes();
    match server.send(message.header.message_id, MESSAGE_TYPE_SHOW_DATABASES, body).await {
        Ok(_) => info!("Databases sent"),
        Err(e) => error!("Failed to send response: {}", e),
    }
}

pub async fn handle_use_database(server: &mut Server, message: &Message) {
    info!("Received USE DATABASE");

    let stm = match UseDatabaseStatement::from_bytes(&message.body) {
        Ok(statement) => statement,
        Err(e) => {
            error!("Failed to parse BSON for USE DATABASE: {}", e);
            return;
        }
    };

    // server.storage().lock().unwrap().use_database(&stm.database_name); en session

    let body = [b"DATABASE USED", stm.database_name.as_bytes()].concat();
    match server.send(message.header.message_id, MESSAGE_TYPE_USE_DATABASE, &body).await {
        Ok(_) => info!("Database used"),
        Err(e) => error!("Failed to send response: {}", e),
    }
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

    let stm = match DropTableStatement::from_bytes(&message.body) {
        Ok(statement) => statement,
        Err(e) => {
            error!("Failed to parse BSON for DROP TABLE: {}", e);
            return;
        }
    };

    server.storage().lock().unwrap().drop_table("", &stm.table_name);

    let body = b"TABLE DROPPED";
    match server.send(message.header.message_id, MESSAGE_TYPE_DROP_TABLE, body).await {
        Ok(_) => info!("Table dropped"),
        Err(e) => error!("Failed to send response: {}", e),
    }
}

pub async fn handle_alter_table(server: &mut Server, message: &Message) {
    info!("Received ALTER TABLE");

    let stm = match AlterTableStatement::from_bytes(&message.body) {
        Ok(statement) => statement,
        Err(e) => {
            error!("Failed to parse BSON for ALTER TABLE: {}", e);
            return;
        }
    };

    server.storage().lock().unwrap().alter_table("", &stm.table_name);

    let body = b"TABLE ALTERED";
    match server.send(message.header.message_id, MESSAGE_TYPE_ALTER_TABLE, body).await {
        Ok(_) => info!("Table altered"),
        Err(e) => error!("Failed to send response: {}", e),
    }
}

pub async fn handle_rename_table(server: &mut Server, message: &Message) {
    info!("Received RENAME TABLE");

    let stm = match RenameTableStatement::from_bytes(&message.body) {
        Ok(statement) => statement,
        Err(e) => {
            error!("Failed to parse BSON for RENAME TABLE: {}", e);
            return;
        }
    };

    server.storage().lock().unwrap().rename_table("", &stm.old_table_name, &stm.new_table_name);

    let body = b"TABLE RENAMED";
    match server.send(message.header.message_id, MESSAGE_TYPE_RENAME_TABLE, body).await {
        Ok(_) => info!("Table renamed"),
        Err(e) => error!("Failed to send response: {}", e),
    }
}

pub async fn handle_truncate_table(server: &mut Server, message: &Message) {
    info!("Received TRUNCATE TABLE");

    let stm = match TruncateTableStatement::from_bytes(&message.body) {
        Ok(statement) => statement,
        Err(e) => {
            error!("Failed to parse BSON for TRUNCATE TABLE: {}", e);
            return;
        }
    };

    server.storage().lock().unwrap().truncate_table("", &stm.table_name);

    let body = b"TABLE TRUNCATED";
    match server.send(message.header.message_id, MESSAGE_TYPE_TRUNCATE_TABLE, body).await {
        Ok(_) => info!("Table truncated"),
        Err(e) => error!("Failed to send response: {}", e),
    }
}

pub async fn handle_show_tables(server: &mut Server, message: &Message) {
    info!("Received SHOW TABLES");

    let tables = server.storage().lock().unwrap().show_tables("");

    let tables_str = tables.join("\n");
    let body = tables_str.as_bytes();
    match server.send(message.header.message_id, MESSAGE_TYPE_SHOW_TABLES, body).await {
        Ok(_) => info!("Tables sent"),
        Err(e) => error!("Failed to send response: {}", e),
    }
}

pub async fn handle_describe_table(server: &mut Server, message: &Message) {
    info!("Received DESCRIBE TABLE");

    let stm = match DescribeTableStatement::from_bytes(&message.body) {
        Ok(statement) => statement,
        Err(e) => {
            error!("Failed to parse BSON for DESCRIBE TABLE: {}", e);
            return;
        }
    };

    let columns = server.storage().lock().unwrap().describe_table("", &stm.table_name);

    let columns_str: String = columns
        .iter()
        .map(|col| format!("{:?}", col))
        .collect::<Vec<String>>()
        .join("\n");
    let body = columns_str.as_bytes();
    match server.send(message.header.message_id, MESSAGE_TYPE_DESCRIBE_TABLE, body).await {
        Ok(_) => info!("Table schema sent"),
        Err(e) => error!("Failed to send response: {}", e),
    }
}

// =====================
// Index Operations
// =====================

pub async fn handle_create_index(server: &mut Server, message: &Message) {
    info!("Received CREATE INDEX");

    let stm = match CreateIndexStatement::from_bytes(&message.body) {
        Ok(statement) => statement,
        Err(e) => {
            error!("Failed to parse BSON for CREATE INDEX: {}", e);
            return;
        }
    };

    server
        .storage()
        .lock()
        .unwrap()
        .create_index("", &stm.index_name, &stm.table_name, &stm.columns, stm.unique);

    let body = b"INDEX CREATED";
    match server.send(message.header.message_id, MESSAGE_TYPE_CREATE_INDEX, body).await {
        Ok(_) => info!("Index created"),
        Err(e) => error!("Failed to send response: {}", e),
    }
}

pub async fn handle_drop_index(server: &mut Server, message: &Message) {
    info!("Received DROP INDEX");

    let stm = match DropIndexStatement::from_bytes(&message.body) {
        Ok(statement) => statement,
        Err(e) => {
            error!("Failed to parse BSON for DROP INDEX: {}", e);
            return;
        }
    };

    server.storage().lock().unwrap().drop_index("", &stm.table_name, &stm.index_name);

    let body = b"INDEX DROPPED";
    match server.send(message.header.message_id, MESSAGE_TYPE_DROP_INDEX, body).await {
        Ok(_) => info!("Index dropped"),
        Err(e) => error!("Failed to send response: {}", e),
    }
}

pub async fn handle_show_indexes(server: &mut Server, message: &Message) {
    info!("Received SHOW INDEXES");

    let stm = match ShowIndexesStatement::from_bytes(&message.body) {
        Ok(statement) => statement,
        Err(e) => {
            error!("Failed to parse BSON for SHOW INDEXES: {}", e);
            return;
        }
    };

    let indexes = server.storage().lock().unwrap().show_indexes("", &stm.table_name);

    let indexes_str = indexes.join("\n");
    let body = indexes_str.as_bytes();

    match server.send(message.header.message_id, MESSAGE_TYPE_SHOW_INDEXES, body).await {
        Ok(_) => info!("Indexes sent"),
        Err(e) => error!("Failed to send response: {}", e),
    }
}

// =====================
// Data Operations
// =====================
pub async fn handle_insert(server: &mut Server, message: &Message) {
    info!("Received INSERT");

    let stm = match InsertStatement::from_bytes(&message.body) {
        Ok(statement) => statement,
        Err(e) => {
            error!("Failed to parse BSON for INSERT: {}", e);
            return;
        }
    };

    server.storage().lock().unwrap().insert("", &stm.table_name, &stm.values);

    let body = b"ROW INSERTED";
    match server.send(message.header.message_id, MESSAGE_TYPE_INSERT, body).await {
        Ok(_) => info!("Row inserted"),
        Err(e) => error!("Failed to send response: {}", e),
    }
}


pub async fn handle_select(server: &mut Server, message: &Message) {
    info!("Received SELECT");

    let stm = match SelectStatement::from_bytes(&message.body) {
        Ok(statement) => statement,
        Err(e) => {
            error!("Failed to parse BSON for SELECT: {}", e);
            return;
        }
    };

    let rows = server.storage().lock().unwrap().select("", &stm.table_name);

    let rows_str: String = rows
        .iter()
        .map(|row| format!("{:?}", row))
        .collect::<Vec<String>>()
        .join("\n");
    let body = rows_str.as_bytes();
    match server.send(message.header.message_id, MESSAGE_TYPE_SELECT, body).await {
        Ok(_) => info!("Rows sent"),
        Err(e) => error!("Failed to send response: {}", e),
    }
}

pub async fn handle_update(server: &mut Server, message: &Message) {
    info!("Received UPDATE");

    let stm = match UpdateStatement::from_bytes(&message.body) {
        Ok(statement) => statement,
        Err(e) => {
            error!("Failed to parse BSON for UPDATE: {}", e);
            return;
        }
    };

    server.storage().lock().unwrap().update("", &stm.table_name, &stm.updates);

    let body = b"ROWS UPDATED";
    match server.send(message.header.message_id, MESSAGE_TYPE_UPDATE, body).await {
        Ok(_) => info!("Rows updated"),
        Err(e) => error!("Failed to send response: {}", e),
    }
}

pub async fn handle_delete(server: &mut Server, message: &Message) {
    info!("Received DELETE");

    let stm = match DeleteStatement::from_bytes(&message.body) {
        Ok(statement) => statement,
        Err(e) => {
            error!("Failed to parse BSON for DELETE: {}", e);
            return;
        }
    };

    server.storage().lock().unwrap().delete("", &stm.table_name);

    let body = b"ROWS DELETED";
    match server.send(message.header.message_id, MESSAGE_TYPE_DELETE, body).await {
        Ok(_) => info!("Rows deleted"),
        Err(e) => error!("Failed to send response: {}", e),
    }
}

pub async fn handle_bulk_insert(server: &mut Server, message: &Message) {
    info!("Received BULK INSERT");

    let stm = match BulkInsertStatement::from_bytes(&message.body) {
        Ok(statement) => statement,
        Err(e) => {
            error!("Failed to parse BSON for BULK INSERT: {}", e);
            return;
        }
    };

    server.storage().lock().unwrap().bulk_insert("", &stm.table_name, &stm.columns, &stm.values);

    let body = b"ROWS INSERTED";
    match server.send(message.header.message_id, MESSAGE_TYPE_BULK_INSERT, body).await {
        Ok(_) => info!("Rows inserted"),
        Err(e) => error!("Failed to send response: {}", e),
    }
}

pub async fn handle_upsert(server: &mut Server, message: &Message) {
    info!("Received UPSERT");

    let stm = match UpdateStatement::from_bytes(&message.body) {
        Ok(statement) => statement,
        Err(e) => {
            error!("Failed to parse BSON for UPSERT: {}", e);
            return;
        }
    };

    server.storage().lock().unwrap().upsert("", &stm.table_name, &stm.updates);

    let body = b"ROWS UPSERTED";
    match server.send(message.header.message_id, MESSAGE_TYPE_UPSERT, body).await {
        Ok(_) => info!("Rows upserted"),
        Err(e) => error!("Failed to send response: {}", e),
    }
}

// =====================
// Transaction Management
// =====================

pub async fn handle_begin_transaction(server: &mut Server, message: &Message) {
    info!("Received BEGIN TRANSACTION");

    let stm = match BeginTransactionStatement::from_bytes(&message.body) {
        Ok(statement) => statement,
        Err(e) => {
            error!("Failed to parse BSON for BEGIN TRANSACTION: {}", e);
            return;
        }
    };

    server.storage().lock().unwrap().begin_transaction(&stm.transaction_id, &stm.isolation_level);

    let body = b"TRANSACTION BEGUN";
    match server.send(message.header.message_id, MESSAGE_TYPE_BEGIN_TRANSACTION, body).await {
        Ok(_) => info!("Transaction begun"),
        Err(e) => error!("Failed to send response: {}", e),
    }
}

pub async fn handle_commit(server: &mut Server, message: &Message) {
    info!("Received COMMIT");

    let body = b"TRANSACTION COMMITTED";
    match server.send(message.header.message_id, MESSAGE_TYPE_COMMIT, body).await {
        Ok(_) => {
            info!("COMMIT sent");
        }
        Err(e) => {
            error!("Failed to send COMMIT: {}", e);
        }
    }
}

pub async fn handle_rollback(server: &mut Server, message: &Message) {
    info!("Received ROLLBACK");

    let body = b"TRANSACTION ROLLED BACK";
    match server.send(message.header.message_id, MESSAGE_TYPE_ROLLBACK, body).await {
        Ok(_) => {
            info!("ROLLBACK sent");
        }
        Err(e) => {
            error!("Failed to send ROLLBACK: {}", e);
        }
    }
}

pub async fn handle_savepoint(server: &mut Server, message: &Message) {
    info!("Received SAVEPOINT");

    let body = b"SAVEPOINT CREATED";
    match server.send(message.header.message_id, MESSAGE_TYPE_SAVEPOINT, body).await {
        Ok(_) => {
            info!("SAVEPOINT sent");
        }
        Err(e) => {
            error!("Failed to send SAVEPOINT: {}", e);
        }
    }
}

pub async fn handle_release_savepoint(server: &mut Server, message: &Message) {
    info!("Received RELEASE SAVEPOINT");

    let body = b"SAVEPOINT RELEASED";
    match server.send(message.header.message_id, MESSAGE_TYPE_RELEASE_SAVEPOINT, body).await {
        Ok(_) => {
            info!("RELEASE SAVEPOINT sent");
        }
        Err(e) => {
            error!("Failed to send RELEASE SAVEPOINT: {}", e);
        }
    }
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

pub async fn handle_greeting(server: &mut Server, message: &Message) {
    info!("Received GREETING");

    let body = b"GREETINGS";
    match server.send(message.header.message_id, MESSAGE_TYPE_GREETING, body).await {
        Ok(_) => {
            info!("GREETINGS sent");
        }
        Err(e) => {
            error!("Failed to send GREETINGS: {}", e);
        }
    }
}

pub async fn handle_welcome(server: &mut Server, message: &Message) {
    info!("Received WELCOME");

    let body = b"WELCOME";
    match server.send(message.header.message_id, MESSAGE_TYPE_WELCOME, body).await {
        Ok(_) => {
            info!("WELCOME sent");
        }
        Err(e) => {
            error!("Failed to send WELCOME: {}", e);
        }
    }
}

pub async fn handle_unknown_command(server: &mut Server, message: &Message) {
    error!("Received UNKNOWN COMMAND");

    let body = b"Unsuppported command";
    match server.send(message.header.message_id, MESSAGE_TYPE_UNKNOWN_COMMAND, body).await {
        Ok(_) => {
            info!("Unknown command sent");
        }
        Err(e) => {
            error!("Failed to send unknown command: {}", e);
        }
    }
}
