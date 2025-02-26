use tokio::io::{ AsyncReadExt, AsyncWriteExt };
use tokio::net::TcpStream;
use std::io;
use tokio::time::{ sleep, Duration };
use crate::network::transport::{ Message, MessageHeader, MessageType };
use async_recursion::async_recursion;

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
    pub async fn send(
        &mut self,
        message_id: [u8; 16],
        message_type: MessageType,
        body: &[u8]
    ) -> Result<(), io::Error> {
        if let Some(stream) = &mut self.stream {
            let header = MessageHeader {
                message_id,
                message_type: message_type as u32,
                body_size: body.len() as u32,
            };

            stream.write_all(&header.to_bytes()).await?;
            stream.write_all(body).await?;

            return Ok(());
        }

        return Err(io::Error::new(io::ErrorKind::NotConnected, "Not connected to the server"));
    }

    /// Receives a message from the server.
    /// Automatically reconnects if the connection is lost.
    pub async fn receive(&mut self) -> Result<Message, io::Error> {
        if let Some(stream) = &mut self.stream {
            // Read the header (24 bytes)
            let mut header_bytes = [0; 24];
            stream.read_exact(&mut header_bytes).await?;

            // Parse the header
            let header = MessageHeader::from_bytes(header_bytes);

            // Read the body based on the size specified in the header
            let mut body = vec![0; header.body_size as usize];
            stream.read_exact(&mut body).await?;

            // Construct and return a Message
            Ok(Message { header, body })
        } else {
            Err(io::Error::new(io::ErrorKind::NotConnected, "Not connected to the server"))
        }
    }

    pub async fn listen(&mut self) {
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
                }
                Err(e) => {
                    println!("Failed to receive message: {}", e);
                    self.connect().await;
                }
            }
        }
    }
}
