mod network;
mod storage;
mod utils;
mod protocol;
mod statement;
mod managment;
mod transport;
mod example;

use example::start_server;

#[tokio::main]
async fn main() {
    start_server().await;
}
