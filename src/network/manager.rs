use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use std::io;
use tokio::time::{sleep, Duration};
use async_recursion::async_recursion;
use super::transport::*;
use std::sync::Arc;
use tokio::sync::{Mutex, Semaphore};

pub struct Manager {
    address: String,
    stream: Option<TcpStream>,
    shared_data: Arc<Mutex<SharedData>>,
    semaphore: Arc<Semaphore>,
}

#[derive(Default)]
struct SharedData {
    counter: u32,
}

impl Manager {
    pub fn new(address: &str) -> Self {
        Self {
            address: address.to_string(),
            stream: None,
            shared_data: Arc::new(Mutex::new(SharedData::default())), 
            semaphore: Arc::new(Semaphore::new(100)), 
        }
    }

    pub fn address(&self) -> &str {
        &self.address
    }

    /// Connects to the server with automatic reconnection.
    /// If the connection is lost, it will attempt to reconnect every 5 seconds.
    pub async fn connect(&mut self) {
        loop {
            match TcpStream::connect(&self.address()).await {
                Ok(stream) => {
                    self.stream = Some(stream);
                    println!("Connected to server at {}", self.address);
                    break;
                }
                Err(e) => {
                    println!(
                        "Failed to connect to {}: {}. Retrying in 5 seconds...",
                        self.address,
                        e
                    );
                    sleep(Duration::from_secs(5)).await;
                }
            }
        }
    }

    /// Sends a message to the server.
    /// Automatically reconnects if the connection is lost.
    #[async_recursion]
    pub async fn send(
        &mut self,
        message_id: [u8; 16],
        message_type: u32,
        body: &[u8]
    ) -> Result<(), io::Error> {
        if let Some(stream) = &mut self.stream {
            let header = MessageHeader {
                message_id,
                message_type: message_type as u32,
                body_size: body.len() as u32,
            };

            let message = Message { header, body: body.to_vec() };
            stream.write_all(&message.to_bytes()).await?;

            return Ok(());
        }

        return Err(io::Error::new(io::ErrorKind::NotConnected, "Not connected to the server"));
    }

    /// Receives a message from the server.
    /// Automatically reconnects if the connection is lost.
    pub async fn receive(&mut self) -> Result<Message, io::Error> {
        if let Some(stream) = &mut self.stream {
            let mut header_bytes = [0; 24];
            stream.read_exact(&mut header_bytes).await?;

            let header = MessageHeader::from_bytes(header_bytes);

            let mut body = vec![0; header.body_size as usize];
            stream.read_exact(&mut body).await?;

            Ok(Message { header, body })
        } else {
            Err(io::Error::new(io::ErrorKind::NotConnected, "Not connected to the server"))
        }
    }

    /// Listens for incoming messages and processes them asynchronously.
    pub async fn listen(&mut self) -> Result<(), io::Error> {
        println!("Listening for messages...");
        loop {
            match self.receive().await {
                Ok(message) => {
                    println!(
                        "Received message (ID: {}, Type: {}, Body: {:?})",
                        hex::encode(message.header.message_id),
                        message.header.message_type,
                        String::from_utf8_lossy(&message.body)
                    );

                    let shared_data = Arc::clone(&self.shared_data);
                    let semaphore = Arc::clone(&self.semaphore);

                    tokio::spawn(async move {
                        let permit = semaphore.acquire().await.unwrap();

                        match message.header.message_type {
                            MESSAGE_TYPE_PING => {
                                println!("Processing PING message");
                                let mut data = shared_data.lock().await;
                                data.counter += 1; 
                                println!("Counter: {}", data.counter);
                            }
                            _ => {
                                println!("Processing unknown message type");
                            }
                        }

                        drop(permit);
                    });
                }
                Err(e) => {
                    println!("Failed to receive message: {}", e);
                    self.connect().await;
                }
            }
        }
    }
}