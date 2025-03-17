use std::collections::BinaryHeap;
use std::sync::{ Arc, Mutex };
use std::time::Duration;
use log::{ info, warn };
use crate::network::{ ZenithConnection, dial_timeout };
use crate::protocol::{ self, MessageType };
use crate::transport::Message;
use crate::statement::LoginStatement;

const RECONNECT_INTERVAL: Duration = Duration::from_secs(3);

#[derive(Debug, Clone)]
pub struct MessageClient {
    server_addr: String,
    token: String,
    node_id: String,
    address: String,
    tags: Vec<String>,
    connections: Arc<Mutex<BinaryHeap<ConnectionPool>>>,
    max_conn: usize,
    min_conn: usize,
    timeout: Duration,
}

#[derive(Debug, Eq, PartialEq)]
struct ConnectionPool {
    conn: ZenithConnection,
    loan_count: usize,
}

impl Ord for ConnectionPool {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.loan_count.cmp(&other.loan_count)
    }
}

impl PartialOrd for ConnectionPool {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone)]
pub struct MessageConfig {
    pub server_addr: String,
    pub token: String,
    pub node_id: String,
    pub address: String,
    pub tags: Vec<String>,
    pub min_conn: usize,
    pub max_conn: usize,
    pub timeout: Duration,
}

#[allow(dead_code)]
impl MessageClient {
    pub async fn new(
        config: MessageConfig
    ) -> Result<MessageClient, Box<dyn std::error::Error + Send + Sync>> {
        let min_conn = config.min_conn.max(1);
        let max_conn = config.max_conn.max(min_conn);

        let client = Self {
            server_addr: config.server_addr,
            token: config.token,
            node_id: config.node_id,
            address: config.address,
            tags: config.tags,
            connections: Arc::new(Mutex::new(BinaryHeap::new())),
            max_conn,
            min_conn,
            timeout: config.timeout,
        };

        client.init_connections().await;

        return Ok(client);
    }

    async fn init_connections(&self) {
        for _ in 0..self.min_conn {
            let _ = self.retry_create_connection();
        }
    }

    async fn retry_create_connection(&self) {
        let connections = self.connections.clone();

        loop {
            let result = self.create_connection().await;
            if let Ok(conn) = result {
                info!("Successfully connected to the server.");
                let mut conn_pool = connections.lock().unwrap();
                conn_pool.push(ConnectionPool {
                    conn,
                    loan_count: 0,
                });
                return;
            }
            warn!("Retrying connection...");

            tokio::time::sleep(RECONNECT_INTERVAL).await;
        }
    }

    async fn create_connection(
        &self
    ) -> Result<ZenithConnection, Box<dyn std::error::Error + Send + Sync>> {
        let result = dial_timeout(&self.server_addr, self.timeout).await;
        let mut conn = match result {
            Ok(conn) => conn,
            Err(e) => {
                return Err(e);
            }
        };

        println!("Connected to the server.");
        self.authenticate(&mut conn).await?;

        return Ok(conn);
    }

    pub async fn allocate_connection(
        &self
    ) -> Result<ZenithConnection, Box<dyn std::error::Error + Send + Sync>> {
        let mut conn_pool = self.connections.lock().unwrap();

        if conn_pool.is_empty() && conn_pool.len() < self.max_conn {
            let _ = self.retry_create_connection().await;
        }

        if conn_pool.is_empty() {
            return Err("No available connections".into());
        }

        let mut selected = conn_pool.pop().unwrap();
        selected.loan_count += 1;

        return Ok(selected.conn);
    }

    pub async fn free_connection(&self, conn: ZenithConnection) {
        let mut connections = self.connections.lock().unwrap();
        let mut temp: Vec<_> = connections.drain().collect();

        for conn_pool in temp.iter_mut() {
            if conn_pool.conn.id == conn.id {
                conn_pool.loan_count -= 1;
                break;
            }
        }

        for item in temp {
            connections.push(item);
        }

        if connections.len() > self.min_conn {
            let _ = self.cleanup_idle_connections(&mut connections).await;
        }
    }

    async fn cleanup_idle_connections(&self, connections: &mut BinaryHeap<ConnectionPool>) {
        let temp: Vec<_> = connections.drain().collect();
        let mut retained = Vec::new();

        for mut conn_pool in temp.into_iter() {
            if retained.len() < self.min_conn || conn_pool.loan_count > 0 {
                retained.push(conn_pool);
            } else {
                let _ = conn_pool.conn.close().await;
            }
        }

        for item in retained {
            connections.push(item);
        }
    }

    pub async fn authenticate(
        &self,
        conn: &mut ZenithConnection
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let stmt = LoginStatement::new(
            self.token.clone(),
            self.node_id.clone(),
            self.node_id.clone(),
            false,
            self.address.clone(),
            self.tags.clone()
        )?;
        let login_message = Message::new(protocol::MessageType::Login, &stmt);
        let response = match conn.send(&login_message).await {
            Ok(response) => response,
            Err(e) => {
                return Err(e);
            }
        };

        if response.header.message_type != MessageType::Login {
            return Err("Authentication failed".into());
        }

        println!("Successfully authenticated with the server.");

        return Ok(());
    }
}
