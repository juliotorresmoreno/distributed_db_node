use super::transport::*;
use crate::Manager;
use log::{info, error};

pub async fn handle_ping(manager: &mut Manager, message: &Message) {
    info!("Received PING");
    info!("Sending PONG");

    let body = b"PONG";
    match manager.send(message.header.message_id, MESSAGE_TYPE_PONG, body).await {
        Ok(_) => {
            info!("PONG sent");
        },
        Err(e) => {
            error!("Failed to send PONG: {}", e);
        },
    }
}

pub async fn handle_create_table(manager: &mut Manager, message: &Message) {
    info!("Received CREATE TABLE");
}

pub async fn handle_drop_table(manager: &mut Manager, message: &Message) {
    info!("Received DROP TABLE");
}

pub async fn handle_alter_table(manager: &mut Manager, message: &Message) {
    info!("Received ALTER TABLE");
}

pub async fn handle_create_index(manager: &mut Manager, message: &Message) {
    info!("Received CREATE INDEX");
}

pub async fn handle_drop_index(manager: &mut Manager, message: &Message) {
    info!("Received DROP INDEX");
}

pub async fn handle_insert(manager: &mut Manager, message: &Message) {
    info!("Received INSERT");
}

pub async fn handle_select(manager: &mut Manager, message: &Message) {
    info!("Received SELECT");
}

pub async fn handle_update(manager: &mut Manager, message: &Message) {
    info!("Received UPDATE");
}

pub async fn handle_delete(manager: &mut Manager, message: &Message) {
    info!("Received DELETE");
}

pub async fn handle_begin_transaction(manager: &mut Manager, message: &Message) {
    info!("Received BEGIN TRANSACTION");
}

pub async fn handle_commit(manager: &mut Manager, message: &Message) {
    info!("Received COMMIT");
}

pub async fn handle_rollback(manager: &mut Manager, message: &Message) {
    info!("Received ROLLBACK");
}

pub async fn handle_greeting(manager: &mut Manager, message: &Message) {
    info!("Received GREETING");
}

pub async fn handle_welcome(manager: &mut Manager, message: &Message) {
    info!("Received WELCOME");
}

pub async fn handle_unknown_command(manager: &mut Manager, message: &Message) {
    info!("Received UNKNOWN COMMAND");
}