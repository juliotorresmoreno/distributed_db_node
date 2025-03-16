use crate::{
    managment::{ MessageClient, MessageConfig },
    transport,
    protocol::message_type::MessageType,
    statement,
};
use scopeguard::defer;
use tokio::time;

const MAX_CONNECTIONS_PER_CLIENT: usize = 200;
const MIN_CONNECTIONS_PER_CLIENT: usize = 100;
const MESSAGES_PER_CLIENT: usize = 10000;
const TIMEOUT: std::time::Duration = std::time::Duration::from_secs(3);
const SERVER_ADDR: &str = "127.0.0.1:8081";
const TOKEN: &str = "my-secure-token";

pub async fn start_server() {
    let clientInstance = MessageClient::new(MessageConfig {
        server_addr: SERVER_ADDR.to_string(),
        token: TOKEN.to_string(),
        node_id: "global_master".to_string(),
        tags: vec!["master".to_string()],
        max_conn: MAX_CONNECTIONS_PER_CLIENT,
        min_conn: MIN_CONNECTIONS_PER_CLIENT,
        timeout: TIMEOUT,
    });

    let mut conn = match clientInstance.allocate_connection().await {
        Ok(conn) => conn,
        Err(e) => {
            println!("Error: {:?}", e);
            return;
        }
    };

    let stmt = match statement::CreateDatabaseStatement::new("test".to_string()) {
        Ok(stmt) => stmt,
        Err(e) => {
            println!("Error: {:?}", e);
            return;
        }
    };
    let msg = transport::Message::new(MessageType::CreateDatabase, &stmt);

    match conn.send(&msg).await {
        Ok(_) => println!("Message sent"),
        Err(e) => println!("Error: {:?}", e),
    }

    time::sleep(std::time::Duration::from_secs(60 * 60)).await;
}
