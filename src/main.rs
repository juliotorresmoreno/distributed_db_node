mod network;
mod storage;
mod utils;
mod protocol;
mod statement;
mod managment;
mod transport;

#[tokio::main]
async fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    println!("Starting server...");
}
