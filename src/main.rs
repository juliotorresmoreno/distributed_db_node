mod network;
mod storage;
mod utils;
mod protocol;
mod managment;

use utils::logger::init_logger;
use utils::config::Config;
use network::server::Server;
use uuid::Uuid;
use tokio::signal;
use tokio::time;
use std::sync::Arc;
use std::process::exit;
use tokio::sync::Notify;
use network::transport::MESSAGE_TYPE_PING;
use storage::engine::Engine as DBEngine;
use managment::client::Client;

#[tokio::main]
async fn main() {
    init_logger();

    let config = Config::load("config.toml").expect("Failed to load config");
    let storage = DBEngine::new();

    let client = Client::new(
        config.management.node_id.clone(),
        config.management.cluster_token.clone(),
        config.management.addr.clone()
    );

    let nodes = client.get_master_nodes().await.expect("Failed to get master nodes");
    let managment_node = tokio::spawn(async move {
        client.connect_to_management().await;
    });

    time::sleep(time::Duration::from_secs(5)).await;

    
    println!("Master nodes: {:?}", nodes);
    println!("Parsed config: {:?}", config);

    // let mut server = Server::new(storage.clone());
    // server.connect(&config.master.addr).await;

    /*
    let message_id = *Uuid::new_v4().as_bytes();
    let message_body = b"Hello, server!";
    // server.send(message_id, MESSAGE_TYPE_PING, message_body).await.expect("Failed to send message");

    let shutdown_signal = Arc::new(Notify::new());
    let shutdown_signal_clone = shutdown_signal.clone();

    /*let listen_handle = tokio::spawn(async move {
        server.listen().await.expect("Failed to start listener");
    });*/

    tokio::select! {
        _ = signal::ctrl_c() => {
            shutdown_signal_clone.notify_one();
            exit(0);
        }
        _ = shutdown_signal.notified() => {}
    }
    */

    let _ = tokio::join!(managment_node);
}
