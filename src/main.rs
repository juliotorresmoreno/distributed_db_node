mod network;
mod storage;
mod api;
mod utils;

use utils::logger::init_logger;
use utils::config::Config;
use network::manager::Manager;

#[tokio::main]
async fn main() {
    init_logger();

    let config = Config::load("config.toml").expect("Failed to load config");
    let storage = storage::kv_store::KVStore::new();
    let server: network::server::Server = network::server::Server::new(config.network.port, storage.clone());
    let api: api::rest::RestApi = api::rest::RestApi::new(config.api.port, storage.clone());

    let mut manager: Manager = Manager::new("localhost:4040");
    match manager.connect().await {
        Ok(_) => println!("Conectado al servidor en {}", manager.address),
        Err(e) => println!("Error al conectar al servidor: {}", e),
    };

    let _ = manager.send("hola mundo").await;
    
    tokio::join!(server.run(), api.run()).0.expect("Failed to run server and API");
}
