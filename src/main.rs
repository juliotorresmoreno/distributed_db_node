mod network;
mod storage;
mod api;
mod utils;

use utils::logger::init_logger;
use utils::config::Config;

#[tokio::main]
async fn main() {
    init_logger();

    let config = Config::load("config.toml").expect("Failed to load config");
    let storage = storage::kv_store::KVStore::new();
    let server = network::server::Server::new(config.network.port, storage.clone());
    let api = api::rest::RestApi::new(config.api.port, storage.clone());

    tokio::join!(server.run(), api.run()).0.expect("Failed to run server and API");
}
