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

pub async fn handle_unknown() {
    println!("Received unknown message type");
}