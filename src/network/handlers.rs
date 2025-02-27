use super::transport::*; 
use crate::Manager; 

pub async fn handle_ping(manager: &mut Manager, message: &Message) {
    println!("Received PING");
    println!("Sending PONG");

    let body = b"PONG";
    match manager.send(message.header.message_id, MESSAGE_TYPE_PONG, body).await {
        Ok(_) => {
            println!("PONG sent");
        },
        Err(e) => {
            println!("Failed to send PONG: {}", e);
        },
    }
}

pub async fn handle_create_table(manager: &mut Manager, message: &Message) {
    println!("Received CREATE TABLE");
}

pub async fn handle_drop_table(manager: &mut Manager, message: &Message) {
    println!("Received DROP TABLE");
}

pub async fn handle_alter_table(manager: &mut Manager, message: &Message) {
    println!("Received ALTER TABLE");
}

pub async fn handle_create_index(manager: &mut Manager, message: &Message) {
    println!("Received CREATE INDEX");
}

pub async fn handle_drop_index(manager: &mut Manager, message: &Message) {
    println!("Received DROP INDEX");
}

pub async fn handle_insert(manager: &mut Manager, message: &Message) {
    println!("Received INSERT");
}

pub async fn handle_select(manager: &mut Manager, message: &Message) {
    println!("Received SELECT");
}

pub async fn handle_update(manager: &mut Manager, message: &Message) {
    println!("Received UPDATE");
}

pub async fn handle_delete(manager: &mut Manager, message: &Message) {
    println!("Received DELETE");
}

pub async fn handle_begin_transaction(manager: &mut Manager, message: &Message) {
    println!("Received BEGIN TRANSACTION");
}

pub async fn handle_commit(manager: &mut Manager, message: &Message) {
    println!("Received COMMIT");
}

pub async fn handle_rollback(manager: &mut Manager, message: &Message) {
    println!("Received ROLLBACK");
}

pub async fn handle_greeting(manager: &mut Manager, message: &Message) {
    println!("Received GREETING");
}

pub async fn handle_welcome(manager: &mut Manager, message: &Message) {
    println!("Received WELCOME");
}

pub async fn handle_unknown_command(manager: &mut Manager, message: &Message) {
    println!("Received UNKNOWN COMMAND");
}
