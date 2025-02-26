use tokio::io::{ AsyncReadExt, AsyncWriteExt };
use tokio::net::TcpStream;
use std::io;
use tokio::time::{ sleep, Duration };
use crate::network::transport::MessageHeader;
use async_recursion::async_recursion;
use hex;

pub struct Manager {
    address: String,
    stream: Option<TcpStream>,
}

impl Manager {
    /// Creates a new Manager with the specified server address.
    pub fn new(address: &str) -> Self {
        Self {
            address: address.to_string(),
            stream: None,
        }
    }

    /// Returns the server address.
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
    pub async fn send(&mut self, message_id: [u8; 16], body: &[u8]) -> Result<(), io::Error> {
        if let Some(stream) = &mut self.stream {
            let header = MessageHeader {
                message_id,
                body_size: body.len() as u32,
            };

            stream.write_all(&header.to_bytes()).await?;
            stream.write_all(body).await?;

            println!(
                "Message sent (ID: {:?}, body size: {} bytes)",
                hex::encode(&header.message_id),
                header.body_size
            );
            return Ok(());
        }

        return Err(io::Error::new(io::ErrorKind::NotConnected, "Not connected to the server"));
    }

    /// Receives a message from the server.
    /// Automatically reconnects if the connection is lost.
    pub async fn receive(&mut self) -> Result<([u8; 16], Vec<u8>), io::Error> {
        if let Some(stream) = &mut self.stream {
            let mut header_bytes = [0; 20];
            stream.read_exact(&mut header_bytes).await?;
            let header = MessageHeader::from_bytes(header_bytes);

            let mut body = vec![0; header.body_size as usize];
            stream.read_exact(&mut body).await?;

            println!(
                "Message received (ID: {:?}, body size: {} bytes)",
                header.message_id,
                header.body_size
            );
            Ok((header.message_id, body))
        } else {
            Err(io::Error::new(io::ErrorKind::NotConnected, "Not connected to the server"))
        }
    }
}
