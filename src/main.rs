mod network;
mod storage;
mod utils;
mod protocol;
mod statement;
mod managment;

use utils::logger::init_logger;


#[tokio::main]
async fn main() {
    init_logger();

    println!("Starting server...");
}
