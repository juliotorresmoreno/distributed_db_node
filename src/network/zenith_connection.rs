use std::collections::HashMap;
use std::sync::atomic::{ AtomicUsize, Ordering };
use std::sync::{ Arc, Mutex };
use tokio::net::TcpStream;
use tokio::sync::Mutex as TokioMutex;
use std::thread::sleep;
use std::time::{ self, Duration };
use tokio::io::{ AsyncReadExt, AsyncWriteExt, ReadHalf };
use tokio::sync::{ mpsc, oneshot, Notify };
use log::{ info, warn, error };
use crate::transport::{ response, Message };
use std::{ ptr, usize };

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

#[allow(dead_code)]
#[derive(Debug)]
pub struct MessageWithResponse {
    pub message: Message,
    pub response_sender: oneshot::Sender<Message>,
}

#[derive(Debug)]
pub struct ZenithConnection {
    pub id: usize,
    require_auth_sender: mpsc::Sender<()>,
    require_auth_receiver: Arc<TokioMutex<mpsc::Receiver<()>>>,
    message_sender: mpsc::Sender<MessageWithResponse>,
}

impl PartialEq for ZenithConnection {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id;
    }
}

impl Eq for ZenithConnection {}

impl Clone for ZenithConnection {
    fn clone(&self) -> Self {
        return ZenithConnection {
            id: self.id,
            message_sender: self.message_sender.clone(),
            require_auth_sender: self.require_auth_sender.clone(),
            require_auth_receiver: self.require_auth_receiver.clone(),
        };
    }
}

#[allow(dead_code)]
impl ZenithConnection {
    fn new(message_sender: mpsc::Sender<MessageWithResponse>) -> ZenithConnection {
        let id = NEXT_ID.fetch_add(1, Ordering::SeqCst);
        let (sender, receiver) = mpsc::channel(1);
        return ZenithConnection {
            id,
            message_sender,
            require_auth_sender: sender,
            require_auth_receiver: Arc::new(TokioMutex::new(receiver)),
        };
    }

    pub async fn send(
        &self,
        message: &Message
    ) -> Result<Message, Box<dyn std::error::Error + Send + Sync>> {
        let (response_sender, response_receiver) = oneshot::channel();

        let message_with_response = MessageWithResponse {
            message: message.clone(),
            response_sender,
        };

        if let Err(e) = self.message_sender.send(message_with_response).await {
            return Err(Box::new(e));
        }

        println!("waiting for response");
        match response_receiver.await {
            Ok(response) => Ok(response),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub async fn on_require_auth(&self) {
        let mut require_auth_receiver = self.require_auth_receiver.lock().await;
        require_auth_receiver.recv().await;
    }

    pub async fn writte() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        return Ok(());
    }

    pub async fn close(&self) {}
}

pub async fn dial_timeout(
    address: &str,
    timeout: Duration
) -> Result<ZenithConnection, Box<dyn std::error::Error + Send + Sync>> {
    let address_cloned = address.to_string();
    let (message_sender, message_receiver) = mpsc::channel(32);
    let id = NEXT_ID.fetch_add(1, Ordering::SeqCst);
    let (sender, receiver) = mpsc::channel(1);

    let config = StartServerConfig {
        address: address_cloned.clone(),
        timeout,
        message_receiver,
        require_auth_sender: sender.clone(),
    };

    tokio::spawn(start_server(config));

    let conn = ZenithConnection {
        id,
        message_sender,
        require_auth_sender: sender,
        require_auth_receiver: Arc::new(TokioMutex::new(receiver)),
    };

    return Ok(conn);
}

struct StartServerConfig {
    address: String,
    timeout: Duration,
    message_receiver: mpsc::Receiver<MessageWithResponse>,
    require_auth_sender: mpsc::Sender<()>,
}

async fn start_server(config: StartServerConfig) {
    let address = config.address;
    let timeout = config.timeout;
    let message_receiver: Arc<TokioMutex<mpsc::Receiver<MessageWithResponse>>> = Arc::new(
        TokioMutex::new(config.message_receiver)
    );
    let response_map = Arc::new(Mutex::new(HashMap::<String, oneshot::Sender<Message>>::new()));
    let require_auth_sender = config.require_auth_sender;

    loop {
        let conn = match TcpStream::connect(address.to_string()).await {
            Ok(conn) => conn,
            Err(e) => {
                error!("Error connecting to server: {:?}", e);
                sleep(Duration::from_secs(timeout.as_secs() as u64));
                continue;
            }
        };
        let _ = conn.set_nodelay(true);

        let (mut reader, mut writer) = tokio::io::split(conn);
        let (tx_close, rx_close) = oneshot::channel::<()>();

        let message_receiver_clone: Arc<TokioMutex<mpsc::Receiver<MessageWithResponse>>> =
            Arc::clone(&message_receiver);

        let response_map_clone = response_map.clone();
        tokio::spawn(async move {
            read_dump(&mut reader, response_map_clone, tx_close).await;
        });

        let response_map_clone = response_map.clone();

        println!("Connection initialized");
        write_dump(&mut writer, message_receiver_clone, response_map_clone, rx_close).await;
        let _ = require_auth_sender.send(()).await;

        println!("Connection closed");
    }
}

async fn write_dump(
    writer: &mut tokio::io::WriteHalf<TcpStream>,
    message_receiver: Arc<TokioMutex<mpsc::Receiver<MessageWithResponse>>>,
    response_map: Arc<Mutex<HashMap<String, oneshot::Sender<Message>>>>,
    rx_close: oneshot::Receiver<()>
) {
    let mut message_receiver = message_receiver.lock().await;

    tokio::pin!(rx_close);

    loop {
        tokio::select! {
            _ = &mut rx_close => {
                break;
            }
            Some(message_with_response) = message_receiver.recv() => {
                response_map
                    .lock()
                    .unwrap()
                    .insert(
                        message_with_response.message.header.message_id_string(),
                        message_with_response.response_sender
                    );

                if let Err(e) = writer.write_all(&message_with_response.message.serialize()).await {
                    error!("Error writing message: {:?}", e);
                    return;
                }

                println!("Sent message: {:?}", message_with_response.message.header.message_id_string());
            }
        }
    }
}

async fn read_dump(
    reader: &mut ReadHalf<TcpStream>,
    response_map: Arc<Mutex<HashMap<String, oneshot::Sender<Message>>>>,
    tx_close: oneshot::Sender<()>
) {
    loop {
        let message = match Message::read_from(reader).await {
            Ok(message) => message,
            Err(e) => {
                error!("Error reading message: {:?}", e);
                if let Err(e) = tx_close.send(()) {
                    error!("Error sending close signal: {:?}", e);
                }
                return;
            }
        };

        let message_id = message.header.message_id_string();
        let response_sender = match response_map.lock().unwrap().remove(&message_id) {
            Some(sender) => sender,
            None => {
                error!("No response sender for message: {:?}", message_id);
                continue;
            }
        };

        if let Err(e) = response_sender.send(message.clone()) {
            error!("Error sending response: {:?}", e);
        }

        println!("Received message: {:?}", message.header.message_id_string());
    }
}
