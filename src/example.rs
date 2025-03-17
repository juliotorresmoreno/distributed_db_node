#![allow(unused_imports)]

use crate::{
    managment::{ MessageClient, MessageConfig },
    transport,
    protocol::message_type::MessageType,
    statement,
};
use tokio::time;

const MAX_CONNECTIONS_PER_CLIENT: usize = 1;
const MIN_CONNECTIONS_PER_CLIENT: usize = 1;
const TIMEOUT: std::time::Duration = std::time::Duration::from_secs(3);
const SERVER_ADDR: &str = "127.0.0.1:8081";
const TOKEN: &str = "my-secure-token";

#[allow(dead_code)]
pub async fn start_server() {
    let client_instance = MessageClient::new(MessageConfig {
        server_addr: SERVER_ADDR.to_string(),
        token: TOKEN.to_string(),
        node_id: "global_master".to_string(),
        address: "".to_string(),
        tags: vec!["master".to_string()],
        max_conn: MAX_CONNECTIONS_PER_CLIENT,
        min_conn: MIN_CONNECTIONS_PER_CLIENT,
        timeout: TIMEOUT,
    });

    println!("Starting server...");
    time::sleep(std::time::Duration::from_secs(1)).await;

    let mut conn = match client_instance.allocate_connection().await {
        Ok(conn) => conn,
        Err(e) => {
            println!("Error: {:?}", e);
            return;
        }
    };

    match client_instance.authenticate(&mut conn).await {
        Ok(_) => println!("Authenticated!"),
        Err(e) => println!("Error: {:?}", e),
    }

    time::sleep(std::time::Duration::from_secs(60 * 60)).await;
}
