mod network;
mod storage;
mod utils;
mod protocol;

use utils::logger::init_logger;
use utils::config::Config;
use network::server::Server;
use uuid::Uuid;
use tokio::signal;
use std::sync::Arc;
use std::process::exit;
use tokio::sync::Notify;
use network::transport::MESSAGE_TYPE_PING;
use storage::engine::Engine as DBEngine;

#[tokio::main]
async fn main() {
    init_logger();

    let config = Config::load("config.toml").expect("Failed to load config");
    let storage = DBEngine::new();

    let mut server = Server::new(storage.clone());
    server.connect(&config.master.addr).await;

    let message_id = *Uuid::new_v4().as_bytes();
    let message_body = b"Hello, server!";
    server
        .send(message_id, MESSAGE_TYPE_PING, message_body).await
        .expect("Failed to send message");

    let shutdown_signal = Arc::new(Notify::new());
    let shutdown_signal_clone = shutdown_signal.clone();

    let listen_handle = tokio::spawn(async move {
        server.listen().await.expect("Failed to start listener");
    });

    tokio::select! {
        _ = signal::ctrl_c() => {
            shutdown_signal_clone.notify_one();
            exit(0);
        }
        _ = shutdown_signal.notified() => {}
    }

    let _ = tokio::join!(listen_handle);
}
