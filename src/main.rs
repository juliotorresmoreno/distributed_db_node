mod network;
mod storage;
mod api;
mod utils;
use utils::logger::init_logger;
use utils::config::Config;
use network::manager::Manager;
use uuid::Uuid;
use crate::network::transport::MessageType;

#[tokio::main]
async fn main() {
    init_logger();

    let config = Config::load("config.toml").expect("Failed to load config");
    let storage = storage::kv_store::KVStore::new();
    let server = network::server::Server::new(config.network.port, storage.clone());
    let api = api::rest::RestApi::new(config.api.port, storage.clone());

    let mut manager = Manager::new("localhost:4040");
    manager.connect().await;

    let message_id = *Uuid::new_v4().as_bytes();
    let message_body = b"Hello, server!";
    manager.send(message_id, MessageType::Ping, message_body)
        .await
        .expect("Failed to send message");

    let listen_handle = tokio::spawn(async move {
        manager.listen().await;
    });

    let (server_result, api_result) = tokio::join!(server.run(), api.run());

    if let Err(e) = server_result {
        eprintln!("Server error: {:?}", e);
    }
    if let Err(e) = api_result {
        eprintln!("API error: {:?}", e);
    }

    let _ = listen_handle.await;
}
