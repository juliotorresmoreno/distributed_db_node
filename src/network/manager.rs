use tokio::io::{ AsyncReadExt, AsyncWriteExt };
use tokio::net::TcpStream;
use std::io;
use tokio::time::{ sleep, Duration };
use async_recursion::async_recursion;
use super::transport::*;
use super::handlers::*;

pub struct Manager {
    address: String,
    stream: Option<TcpStream>,
}

impl Manager {
    pub fn new(address: &str) -> Self {
        Self {
            address: address.to_string(),
            stream: None,
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
                    match message.header.message_type {
                        MESSAGE_TYPE_PING => handle_ping(self, &message).await,
                        _ => handle_unknown().await,
                    }
                }
                Err(e) => {
                    println!("Failed to receive message: {}", e);
                    self.connect().await;
                }
            }
        }
    }
}
