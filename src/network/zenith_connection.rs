use std::collections::HashMap;
use std::sync::{ Arc, Mutex };
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::io::{ AsyncReadExt, AsyncWriteExt, ReadHalf };
use tokio::sync::{ oneshot, Notify };
use log::{ error, warn };
use crate::transport::Message;
use std::ptr;

#[derive(Debug)]
pub struct ZenithConnection {
    writer: Option<tokio::io::WriteHalf<TcpStream>>,
    response_map: Arc<Mutex<HashMap<String, oneshot::Sender<Message>>>>,
    timeout: Duration,
    pub id: usize,
    on_close: Arc<Notify>,
}

impl PartialEq for ZenithConnection {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id;
    }
}

impl Eq for ZenithConnection {}

impl Clone for ZenithConnection {
    fn clone(&self) -> Self {
        Self {
            writer: None,
            response_map: self.response_map.clone(),
            timeout: self.timeout,
            id: self.id,
            on_close: self.on_close.clone(),
        }
    }
}

#[allow(dead_code)]
impl ZenithConnection {
    pub fn new(conn: TcpStream, timeout: Duration) -> Self {
        let id = ptr::addr_of!(conn) as usize;
        let (mut reader, writer) = tokio::io::split(conn);

        let connection = ZenithConnection {
            writer: Some(writer),
            response_map: Arc::new(Mutex::new(HashMap::new())),
            timeout,
            id,
            on_close: Arc::new(Notify::new()),
        };

        let connection_clone = connection.clone();
        let notify_clone = connection.on_close.clone();
        tokio::spawn(async move {
            connection_clone.listen(&mut reader, notify_clone).await;
        });

        return connection;
    }

    async fn listen(self, reader: &mut ReadHalf<TcpStream>, on_close: Arc<Notify>) {
        loop {
            match Message::read_from(reader).await {
                Ok(message) => {
                    let message_id = message.header.message_id_string();
                    let mut response_map = self.response_map.lock().unwrap();

                    if let Some(tx) = response_map.remove(&message_id) {
                        if tx.send(message).is_err() {
                            warn!("Failed to send response to receiver");
                        }
                    } else {
                        warn!("Received unexpected message: {:?}", message_id);
                    }
                }
                Err(e) => {
                    error!("Connection closed due to error: {:?}", e);
                    on_close.notify_waiters();
                    break;
                }
            }
        }
    }

    pub async fn on_close(&self) {
        self.on_close.notified().await;
    }

    pub async fn send(
        &mut self,
        message: &Message
    ) -> Result<Message, Box<dyn std::error::Error + Send + Sync>> {
        let message_id = message.header.message_id_string();
        let (tx, rx) = oneshot::channel();
        let writer = self.writer.as_mut().unwrap();

        {
            let mut response_map = self.response_map.lock().unwrap();
            response_map.insert(message_id.clone(), tx);
        }

        writer.write_all(&message.serialize()).await?;

        let response = tokio::time::timeout(self.timeout, rx).await;
        match response {
            Ok(Ok(response)) => Ok(response),
            Ok(Err(e)) => Err(Box::new(e)),
            Err(_) => {
                self.cleanup_response_map(&message_id);
                Err(
                    Box::new(
                        std::io::Error::new(
                            std::io::ErrorKind::TimedOut,
                            "timeout waiting for response"
                        )
                    )
                )
            }
        }
    }

    fn cleanup_response_map(&self, message_id: &str) {
        let mut response_map = self.response_map.lock().unwrap();
        response_map.remove(message_id);
    }

    pub async fn close(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(mut writer) = self.writer.take() {
            writer.shutdown().await?;
            self.on_close.notify_waiters();
        }
        Ok(())
    }
}

pub async fn dial_timeout(
    address: &str,
    timeout: Duration
) -> Result<ZenithConnection, Box<dyn std::error::Error + Send + Sync>> {
    let conn = TcpStream::connect(address).await?;
    conn.set_nodelay(true)?;

    return Ok(ZenithConnection::new(conn, timeout));
}
