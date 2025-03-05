mod network;
mod storage;
mod utils;
mod protocol;
mod managment;

use utils::logger::init_logger;
use utils::config::Config;
use network::server::Server;
use std::sync::Arc;
use storage::engine::Engine as DBEngine;
use managment::client::Client;

#[tokio::main]
async fn main() {
    init_logger();

    let config = Config::load("config.toml").expect("Failed to load config");

    let event_handler = Arc::new(|nodes: Vec<String>| {
        let storage = DBEngine::new();

        for node in nodes {
            let storage_clone = storage.clone();
            tokio::spawn(async move {
                let mut server: Server = Server::new(storage_clone);
                server.connect(&node.replace("tcp://", "")).await;
                if let Err(e) = server.listen().await {
                    eprintln!("Failed to start listener for {}: {}", node, e);
                }
            });
        }
    });

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
