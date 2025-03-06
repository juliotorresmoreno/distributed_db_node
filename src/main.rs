mod network;
mod storage;
mod utils;
mod protocol;
mod managment;

use utils::logger::init_logger;
use utils::config::Config;
use network::server::Server;
use std::sync::{ Arc, Mutex };
use storage::engine::Engine as DBEngine;
use managment::client::Client;
use tokio::task::JoinHandle;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    init_logger();

    let config = Config::load("config.toml").expect("Failed to load config");

    let active_connections: Arc<Mutex<HashMap<String, JoinHandle<()>>>> = Arc::new(
        Mutex::new(HashMap::new())
    );
    let storage = Arc::new(DBEngine::new());

    let event_handler = {
        let active_connections = Arc::clone(&active_connections);
        let storage = Arc::clone(&storage);

        Arc::new(move |nodes: Vec<String>| {
            let mut active_nodes = active_connections.lock().unwrap();

            let new_nodes: Vec<String> = nodes.clone();
            let existing_nodes: Vec<String> = active_nodes.keys().cloned().collect();

            for node in &existing_nodes {
                if !new_nodes.contains(node) {
                    if let Some(handle) = active_nodes.remove(node) {
                        handle.abort();
                        println!("Disconnected from master: {}", node);
                    }
                }
            }

            for node in new_nodes {
                if !active_nodes.contains_key(&node) {
                    let storage_clone = Arc::clone(&storage);
                    let node_clone = node.clone();

                    let handle: JoinHandle<()> = tokio::spawn(async move {
                        let mut server = Server::new(Arc::clone(&storage_clone));
                        server.connect(&node_clone.replace("tcp://", "")).await;
                        if let Err(e) = server.listen().await {
                            eprintln!("Failed to start listener for {}: {}", node_clone, e);
                        }
                    });

                    active_nodes.insert(node.clone(), handle);
                    println!("Connected to new master: {}", node);
                }
            }
        })
    };

    let client = Client::new(
        config.management.node_id.clone(),
        config.management.cluster_token.clone(),
        config.management.addr.clone(),
        config.management.url.clone()
    ).with_event_handler(event_handler);

    let managment_node = tokio::spawn(async move {
        client.connect_to_management().await;
    });

    let _ = tokio::join!(managment_node);
}
