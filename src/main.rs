#[allow(unused_imports)]
mod network;
#[allow(unused_imports)]
mod storage;
#[allow(unused_imports)]
mod utils;
#[allow(unused_imports)]
mod protocol;
#[allow(unused_imports)]
mod statement;
#[allow(unused_imports)]
mod managment;
#[allow(unused_imports)]
mod transport;
#[allow(unused_imports)]
mod example;

use example::start_server;

#[tokio::main]
async fn main() {
    start_server().await;
}
