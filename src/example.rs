#![allow(unused_imports)]

use std::sync::Arc;
use tokio::sync::Mutex;
use crate::{
    managment::{ MessageClient, MessageConfig },
    protocol::message_type::MessageType,
    statement::{ self, CreateDatabaseStatement },
    transport::{ self, Message },
};
use tokio::time;

const NUM_CLIENTS: usize = 50;
const MESSAGES_PER_CLIENT: usize = 10000;
const MAX_CONNECTIONS_PER_CLIENT: usize = 10;
const MIN_CONNECTIONS_PER_CLIENT: usize = 100;
const TIMEOUT: std::time::Duration = std::time::Duration::from_secs(3);
const SERVER_ADDR: &str = "127.0.0.1:8081";
const TOKEN: &str = "my-secure-token";

#[allow(dead_code)]
pub async fn start_server() {
    let result = MessageClient::new(MessageConfig {
        server_addr: SERVER_ADDR.to_string(),
        token: TOKEN.to_string(),
        node_id: "slave_0".to_string(),
        address: "".to_string(),
        tags: vec!["slave".to_string()],
        max_conn: MAX_CONNECTIONS_PER_CLIENT,
        min_conn: MIN_CONNECTIONS_PER_CLIENT,
        timeout: TIMEOUT,
    }).await;
    if let Err(e) = result {
        println!("Error: {:?}", e);
        return;
    }

    println!("Server started!");

    let client_instance = match result {
        Ok(client_instance) => client_instance,
        Err(e) => {
            println!("Error: {:?}", e);
            return;
        }
    };

    println!("Starting server...");

    // Trying to get a connection
    #[allow(unused_variables)]
    let conn = match client_instance.allocate_connection().await {
        Ok(conn) => conn,
        Err(e) => {
            println!("Error: {:?}", e);
            return;
        }
    };

    // Send a message
    let stmt = match CreateDatabaseStatement::new("my_database".to_string()) {
        Ok(stmt) => stmt,
        Err(e) => {
            println!("Error: {:?}", e);
            return;
        }
    };
    let message = Message::new(MessageType::CreateDatabase, &stmt);
    let result = conn.send(&message).await;
    if let Err(e) = result {
        println!("Error: {:?}", e);
        return;
    }
    client_instance.free_connection(conn);

    let successful_requests = Arc::new(Mutex::new(0));
    let failed_requests = Arc::new(Mutex::new(0));
    println!("Server started!");
    let start_test_time = time::Instant::now();

    let mut handles: Vec<tokio::task::JoinHandle<()>> = Vec::new();
    for _ in 0..NUM_CLIENTS {
        let successful_requests_clone = Arc::clone(&successful_requests);
        let failed_requests_clone = Arc::clone(&failed_requests);

        let client_instance = client_instance.clone();
        let handle = tokio::spawn(async move {
            match start_client(client_instance).await {
                Ok(_) => {
                    let mut successful_requests = successful_requests_clone.lock().await;
                    *successful_requests += 1;
                }
                Err(_) => {
                    let mut failed_requests = failed_requests_clone.lock().await;
                    *failed_requests += 1;
                }
            }
        });

        handles.push(handle);
    }

    futures::future::join_all(handles).await;

    let end_test_time = time::Instant::now();
    let elapsed_time = end_test_time - start_test_time;
    let requests_per_second =
        (NUM_CLIENTS * MESSAGES_PER_CLIENT) / (elapsed_time.as_secs() as usize);

    let successful_requests = successful_requests.lock().await;
    let failed_requests = failed_requests.lock().await;

    println!("=== Load Test Results ===");
    println!("Number of clients: {}", NUM_CLIENTS);
    println!("Number of messages per client: {}", MESSAGES_PER_CLIENT);
    println!("Number of connections per client: {}", MAX_CONNECTIONS_PER_CLIENT);
    println!("Number of successful requests: {}", *successful_requests);
    println!("Number of failed requests: {}", *failed_requests);
    println!("Total requests sent: {}", NUM_CLIENTS * MESSAGES_PER_CLIENT);
    println!("Elapsed time: {:?}", elapsed_time);
    println!("requests per second: {}", requests_per_second);


}

async fn start_client(
    client_instance: MessageClient
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Trying to get a connection
    #[allow(unused_variables)]
    let conn = match client_instance.allocate_connection().await {
        Ok(conn) => conn,
        Err(e) => {
            println!("Error: {:?}", e);
            return Err(e);
        }
    };

    let database_name = "my_database".to_string();

    for _ in 0..MESSAGES_PER_CLIENT {
        let stmt: CreateDatabaseStatement = match
            CreateDatabaseStatement::new(database_name.clone())
        {
            Ok(stmt) => stmt,
            Err(e) => {
                println!("Error: {:?}", e);
                return Err(Box::new(e));
            }
        };
        let message = Message::new(MessageType::CreateDatabase, &stmt);
        let result = conn.send(&message).await;
        if let Err(e) = result {
            println!("Error: {:?}", e);
            return Err(e);
        }
    }
    
    client_instance.free_connection(conn);

    return Ok(());
}
