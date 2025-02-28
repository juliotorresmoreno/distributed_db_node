use tokio::net::{ TcpListener, TcpStream };
use std::sync::{ Arc, Mutex };
use crate::storage::dbengine::DBEngine;
use std::fmt;
use log::{ info, error };

pub struct Server {
    port: u16,
    storage: Arc<Mutex<DBEngine>>,
}

impl Server {
    pub fn new(port: u16, storage: Arc<Mutex<DBEngine>>) -> Self {
        Self { port, storage }
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", self.port)).await?;
        info!("TCP Server running on port {}", self.port);

        loop {
            let (socket, addr) = listener.accept().await?;
            info!("New connection from {}", addr);

            let storage = Arc::clone(&self.storage);

            tokio::spawn(async move {
                if let Err(e) = handle_connection(socket, storage).await {
                    error!("Connection error: {}", e);
                }
            });
        }
    }
}

async fn handle_connection(
    mut socket: TcpStream,
    storage: Arc<Mutex<DBEngine>>
) -> Result<(), Box<dyn std::error::Error>> {

    Ok(())
}

impl fmt::Debug for Server {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Server").field("port", &self.port).finish()
    }
}
