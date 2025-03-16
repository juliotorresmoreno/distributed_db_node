use std::collections::HashMap;
use std::sync::{ Arc, Mutex };
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::io::{ AsyncReadExt, AsyncWriteExt };
use tokio::sync::oneshot;
use log::{ error, warn };
use crate::transport::Message;
use std::ptr;

#[derive(Debug)]
pub struct ZenithConnection {
    conn: tokio::net::TcpStream,
    response_map: Arc<Mutex<HashMap<String, oneshot::Sender<Message>>>>,
    timeout: Duration,
    close_signal: Arc<tokio::sync::Mutex<Option<tokio::sync::broadcast::Sender<()>>>>,
    id: usize, // Unique identifier for each connection based on memory address
}

impl Eq for ZenithConnection {}

impl ZenithConnection {
    pub async fn connect(
        address: &str,
        timeout: Duration
    ) -> Result<ZenithConnection, Box<dyn std::error::Error>> {
        let conn = TcpStream::connect(address).await?;
        conn.set_nodelay(true)?;
        let id = ptr::addr_of!(conn) as usize;

        return Ok(ZenithConnection {
            conn,
            response_map: Arc::new(Mutex::new(HashMap::new())),
            timeout,
            close_signal: Arc::new(tokio::sync::Mutex::new(None)),
            id,
        });
    }

    pub fn new(conn: TcpStream, timeout: Duration) -> Self {
        let id = ptr::addr_of!(conn) as usize;
        return ZenithConnection {
            conn,
            response_map: Arc::new(Mutex::new(HashMap::new())),
            timeout,
            close_signal: Arc::new(tokio::sync::Mutex::new(None)),
            id,
        };
    }
    pub async fn send(&mut self, message: Message) -> Result<Message, Box<dyn std::error::Error + Send + Sync>> {
        let message_id = message.header.message_id_string();
        let (tx, rx) = oneshot::channel();
    
        {
            let mut response_map = self.response_map.lock().unwrap();
            response_map.insert(message_id.clone(), tx);
        }
    
        self.conn.write_all(&message.serialize()).await?;
    
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

    pub async fn listen_with_callback(
        &mut self,
        on_close: Option<impl FnOnce(Box<dyn std::error::Error>)>
    ) {
        let mut buf = vec![0; 1024]; // Adjust buffer size accordingly
        loop {
            match self.conn.read(&mut buf).await {
                Ok(0) => {
                    break;
                }
                Ok(n) => {
                    let message = Message::deserialize(&buf[..n]).unwrap();
                    let message_id = message.header.message_id_string();
                    if let Some(response_chan) = self.get_response_channel(&message_id) {
                        response_chan.send(message).unwrap();
                    } else {
                        warn!("Received unexpected message: {:?}", message.header.message_type);
                    }
                }
                Err(e) => {
                    if let Some(callback) = on_close {
                        callback(Box::new(e));
                    }
                    break;
                }
            }
        }
    }

    fn get_response_channel(&self, message_id: &str) -> Option<oneshot::Sender<Message>> {
        let mut response_map = self.response_map.lock().unwrap();
        response_map.remove(message_id)
    }

    pub async fn listen(&mut self) {
        let mut buf = vec![0; 1024]; // Adjust buffer size accordingly
        loop {
            match self.conn.read(&mut buf).await {
                Ok(0) => {
                    break;
                }
                Ok(n) => {
                    let message = Message::deserialize(&buf[..n]).unwrap();
                    let message_id = message.header.message_id_string();
                    if let Some(response_chan) = self.get_response_channel(&message_id) {
                        response_chan.send(message).unwrap();
                    }
                }
                Err(e) => {
                    error!("Error reading message: {:?}", e);
                    break;
                }
            }
        }
    }

    pub async fn close(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut close_signal = self.close_signal.lock().await;
        if let Some(signal) = close_signal.take() {
            signal.send(()).unwrap();
            self.conn.shutdown().await?;
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