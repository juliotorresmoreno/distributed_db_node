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
const MAX_CONNECTIONS_PER_CLIENT: usize = 100;
const MIN_CONNECTIONS_PER_CLIENT: usize = 200;
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
            let _ = start_client(
                client_instance,
                successful_requests_clone,
                failed_requests_clone
            ).await;
        });

        handles.push(handle);
    }

    futures::future::join_all(handles).await;

    let end_test_time = time::Instant::now();
    let mut elapsed_time = end_test_time - start_test_time;
    let total_requests = NUM_CLIENTS * MESSAGES_PER_CLIENT;
    if elapsed_time.as_secs() == 0 {
        elapsed_time = time::Duration::from_secs(1);
    }
    let requests_per_second = total_requests / (elapsed_time.as_millis() as usize / 1000);

    let successful_requests = successful_requests.lock().await;
    let failed_requests = failed_requests.lock().await;

    println!("=== Load Test Results ===");
    println!("Number of clients: {}", NUM_CLIENTS);
    println!("Number of messages per client: {}", MESSAGES_PER_CLIENT);
    println!("Number of connections per client: {}", MAX_CONNECTIONS_PER_CLIENT);
    println!("Number of successful requests: {}", *successful_requests);
    println!("Number of failed requests: {}", *failed_requests);
    println!("Total requests sent: {}", total_requests);
    println!("Elapsed time: {:?}", elapsed_time);
    println!("requests per second: {}", requests_per_second);
}

async fn start_client(
    client_instance: MessageClient,
    successfull_requests: Arc<Mutex<usize>>,
    failed_requests: Arc<Mutex<usize>>
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

    let mut handlers: Vec<tokio::task::JoinHandle<()>> = Vec::new();

    for i in 0..MESSAGES_PER_CLIENT {
        let conn_clone = conn.clone();
        let database_name = format!("{}_{}", database_name, i);
        let successful_requests_clone = successfull_requests.clone();
        let failed_requests_clone = failed_requests.clone();
        let handler = tokio::spawn(async move {
            let stmt = match CreateDatabaseStatement::new(database_name) {
                Ok(stmt) => stmt,
                Err(e) => {
                    println!("Error: {:?}", e);
                    return;
                }
            };
            let message = Message::new(MessageType::CreateTable, &stmt);
            let result = conn_clone.send(&message).await;
            if let Err(e) = result {
                let mut failed_requests = failed_requests_clone.lock().await;
                *failed_requests += 1;

                println!("Error: {:?}", e);
                return;
            } else {
                let mut successful_requests = successful_requests_clone.lock().await;
                *successful_requests += 1;
            }
        });

        handlers.push(handler);
    }

    futures::future::join_all(handlers).await;

    client_instance.free_connection(conn);

    return Ok(());
}
