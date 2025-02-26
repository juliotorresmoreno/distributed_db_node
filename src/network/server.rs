use tokio::net::{ TcpListener, TcpStream };
use tokio::io::{ AsyncReadExt, AsyncWriteExt };
use std::sync::{ Arc, Mutex };
use crate::storage::kv_store::KVStore;
use std::fmt;

pub struct Server {
    port: u16,
    storage: Arc<Mutex<KVStore>>,
}

impl Server {
    pub fn new(port: u16, storage: Arc<Mutex<KVStore>>) -> Self {
        Self { port, storage }
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", self.port)).await?;
        println!("TCP Server running on port {}", self.port);

        loop {
            let (socket, addr) = listener.accept().await?;
            println!("New connection from {}", addr);

            let storage = Arc::clone(&self.storage);

            tokio::spawn(async move {
                if let Err(e) = handle_connection(socket, storage).await {
                    println!("Connection error: {}", e);
                }
            });
        }
    }
}

async fn handle_connection(
    mut socket: TcpStream,
    storage: Arc<Mutex<KVStore>>
) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = [0u8; 1024];
    let bytes_read = socket.read(&mut buffer).await?;

    if bytes_read == 0 {
        return Ok(());
    }

    let request = String::from_utf8_lossy(&buffer[..bytes_read]);
    println!("Request received: {}", request);

    let response = if request.starts_with("GET") {
        let key = request.trim_start_matches("GET ").trim();
        let value = storage
            .lock()
            .unwrap()
            .get(key)
            .unwrap_or_else(|| "Key not found".to_string());
        format!("VALUE: {}\n", value)
    } else if request.starts_with("SET") {
        let parts: Vec<&str> = request.trim_start_matches("SET ").splitn(2, ' ').collect();
        if parts.len() == 2 {
            storage.lock().unwrap().set(parts[0].to_string(), parts[1].to_string());
            "OK\n".to_string()
        } else {
            "Invalid SET command\n".to_string()
        }
    } else {
        "Invalid command\n".to_string()
    };

    socket.write_all(response.as_bytes()).await?;
    socket.flush().await?;

    Ok(())
}

impl fmt::Debug for Server {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Server").field("port", &self.port).finish()
    }
}
