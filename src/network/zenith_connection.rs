use std::collections::HashMap;
use std::sync::{ Arc, Mutex };
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::io::{ AsyncReadExt, AsyncWriteExt, ReadHalf };
use tokio::sync::oneshot;
use log::{ error, warn };
use crate::transport::Message;
use std::ptr;

#[allow(dead_code)]
#[derive(Debug)]
pub struct ZenithConnection {
    writer: Option<tokio::io::WriteHalf<TcpStream>>,
    response_map: Arc<Mutex<HashMap<String, oneshot::Sender<Message>>>>,
    timeout: Duration,
    close_signal: Arc<tokio::sync::Mutex<Option<tokio::sync::broadcast::Sender<()>>>>,
    pub id: usize,
}

impl Eq for ZenithConnection {}
impl Clone for ZenithConnection {
    fn clone(&self) -> Self {
        ZenithConnection {
            writer: None,
            response_map: self.response_map.clone(),
            timeout: self.timeout,
            close_signal: self.close_signal.clone(),
            id: self.id,
        }
    }
}

#[allow(dead_code)]
impl ZenithConnection {
    pub async fn connect(
        address: &str,
        timeout: Duration
    ) -> Result<ZenithConnection, Box<dyn std::error::Error>> {
        let conn = TcpStream::connect(address).await?;
        conn.set_nodelay(true)?;
        let connection = ZenithConnection::new(conn, timeout);

        return Ok(connection);
    }

    pub fn new(conn: TcpStream, timeout: Duration) -> Self {
        let id = ptr::addr_of!(conn) as usize;
        let (mut reader, writer) = tokio::io::split(conn);

        let connection = ZenithConnection {
            writer: Some(writer),
            response_map: Arc::new(Mutex::new(HashMap::new())),
            timeout: timeout,
            close_signal: Arc::new(tokio::sync::Mutex::new(None)),
            id: id,
        };

        let connection_clone = connection.clone();
        tokio::spawn(async move {
            connection_clone.listen(&mut reader).await;
        });

        return connection;
    }

    async fn listen(self, reader: &mut ReadHalf<TcpStream>) {
        loop {
            let message = match Message::read_from(reader).await {
                Ok(message) => message,
                Err(e) => {
                    error!("Error reading message: {:?}", e);
                    break;
                }
            };

            let message_id = message.header.message_id_string();
            {
                let mut response_map = self.response_map.lock().unwrap();

                if let Some(tx) = response_map.remove(&message_id) {
                    if let Err(e) = tx.send(message) {
                        warn!("Error sending message: {:?}", e);
                    }
                } else {
                    warn!("Received unexpected message: {:?}", message_id);
                }
            };
        }
    }

    pub async fn send(
        &mut self,
        message: &Message
    ) -> Result<Message, Box<dyn std::error::Error + Send + Sync>> {
        let message_id = message.header.message_id_string();
        let (tx, rx) = oneshot::channel();
        let writter = self.writer.as_mut().unwrap();

        {
            let mut response_map = self.response_map.lock().unwrap();
            response_map.insert(message_id.clone(), tx);
        }

        writter.write_all(&message.serialize()).await?;

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
        let mut close_signal = self.close_signal.lock().await;
        let writer = self.writer.as_mut().unwrap();
        if let Some(signal) = close_signal.take() {
            signal.send(()).unwrap();
            writer.shutdown().await?;
        }
        Ok(())
    }
}

impl PartialEq for ZenithConnection {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

pub async fn dial_timeout(
    address: &str,
    timeout: Duration
) -> Result<ZenithConnection, Box<dyn std::error::Error + Send + Sync>> {
    let conn = TcpStream::connect(address).await?;
    conn.set_nodelay(true)?;

    Ok(ZenithConnection::new(conn, timeout))
}
