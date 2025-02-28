use tokio::io::{ AsyncReadExt, AsyncWriteExt };
use tokio::net::TcpStream;
use std::io;
use tokio::time::{ sleep, Duration };
use async_recursion::async_recursion;
use std::sync::{ Arc, Mutex };
use crate::storage::dbengine::DBEngine;

use super::transport::*;
use super::handlers::*;
use log::{ info, error };

pub struct Server {
    address: Option<String>,
    stream: Option<TcpStream>,
    storage: Arc<Mutex<DBEngine>>,
}

impl Server {
    pub fn new(storage: Arc<Mutex<DBEngine>>) -> Self {
        return Self {
            address: None,
            stream: None,
            storage: storage,
        };
    }

    pub fn storage(&self) -> Arc<Mutex<DBEngine>> {
        return Arc::clone(&self.storage);
    }

    pub fn address(&self) -> Option<&str> {
        return self.address.as_deref();
    }

    pub async fn connect(&mut self, address: &str) {
        self.address = Some(address.to_string());
        loop {
            match TcpStream::connect(address).await {
                Ok(stream) => {
                    self.stream = Some(stream);
                    info!("Connected to server at {}", address);
                    break;
                }
                Err(e) => {
                    error!("Failed to connect to {}: {}. Retrying in 5 seconds...", address, e);
                    sleep(Duration::from_secs(5)).await;
                }
            }
        }
    }

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

    pub async fn listen(&mut self) -> Result<(), io::Error> {
        info!("Listening for messages...");
        loop {
            match self.receive().await {
                Ok(message) => self.handle_message(&message).await,
                Err(e) => {
                    error!("Failed to receive message: {}", e);
                    let address = self.address().unwrap().to_string();
                    self.connect(&address).await;
                }
            }
        }
    }

    pub async fn handle_message(&mut self, message: &Message) {
        match message.header.message_type {
            MESSAGE_TYPE_CREATE_DATABASE => handle_create_database(self, &message).await,
            MESSAGE_TYPE_DROP_DATABASE => handle_drop_database(self, &message).await,
            MESSAGE_TYPE_SHOW_DATABASES => handle_show_databases(self, &message).await,
            MESSAGE_TYPE_PING => handle_ping(self, &message).await,
            MESSAGE_TYPE_CREATE_TABLE => handle_create_table(self, &message).await,
            MESSAGE_TYPE_DROP_TABLE => handle_drop_table(self, &message).await,
            MESSAGE_TYPE_ALTER_TABLE => handle_alter_table(self, &message).await,
            MESSAGE_TYPE_CREATE_INDEX => handle_create_index(self, &message).await,
            MESSAGE_TYPE_DROP_INDEX => handle_drop_index(self, &message).await,
            MESSAGE_TYPE_INSERT => handle_insert(self, &message).await,
            MESSAGE_TYPE_SELECT => handle_select(self, &message).await,
            MESSAGE_TYPE_UPDATE => handle_update(self, &message).await,
            MESSAGE_TYPE_DELETE => handle_delete(self, &message).await,
            MESSAGE_TYPE_BEGIN_TRANSACTION => handle_begin_transaction(self, &message).await,
            MESSAGE_TYPE_COMMIT => handle_commit(self, &message).await,
            MESSAGE_TYPE_ROLLBACK => handle_rollback(self, &message).await,
            MESSAGE_TYPE_GREETING => handle_greeting(self, &message).await,
            MESSAGE_TYPE_WELCOME => handle_welcome(self, &message).await,
            MESSAGE_TYPE_UNKNOWN_COMMAND => handle_unknown_command(self, &message).await,
            _ => handle_unknown_command(self, &message).await,
        }
    }
}
