mod network;
mod storage;
mod utils;
mod protocol;

use utils::logger::init_logger;
use utils::config::Config;
use network::manager::Manager;
use uuid::Uuid;
use tokio::signal;
use std::sync::Arc;
use std::process::exit;
use tokio::sync::Notify;
use network::transport::MESSAGE_TYPE_PING;

#[tokio::main]
async fn main() {
    init_logger();

    let config = Config::load("config.toml").expect("Failed to load config");
    let storage = storage::dbengine::DBEngine::new();
    let server = network::server::Server::new(config.network.port, storage.clone());

    let mut manager = Manager::new("localhost:4040", storage.clone());
    manager.connect().await;

    let message_id = *Uuid::new_v4().as_bytes();
    let message_body = b"Hello, server!";
    manager
        .send(message_id, MESSAGE_TYPE_PING, message_body).await
        .expect("Failed to send message");

    let shutdown_signal = Arc::new(Notify::new());
    let shutdown_signal_clone = shutdown_signal.clone();

    let listen_handle = tokio::spawn(async move {
        manager.listen().await.expect("Failed to start listener");
    });

    let server_handle = tokio::spawn(async move {
        server.run().await.expect("Failed to start server");
    });

    tokio::select! {
        _ = signal::ctrl_c() => {
            shutdown_signal_clone.notify_one();
            exit(0);
        }
        _ = shutdown_signal.notified() => {}
    }

    let _ = tokio::join!(listen_handle, server_handle);
}
