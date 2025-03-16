use std::collections::BinaryHeap;
use std::sync::{ Arc, Mutex };
use std::time::Duration;
use futures::FutureExt;
use tokio::net::TcpStream;
use tokio::sync::oneshot;
use log::{ info, warn };
use crate::network::{ ZenithConnection, dial_timeout }; // Assume your ZenithConnection is in network
use crate::protocol::{ self, MessageType };
use crate::transport::{ Message };
use crate::statement::{ self, LoginStatement };

const RECONNECT_INTERVAL: Duration = Duration::from_secs(3);

#[derive(Debug, Clone)]
pub struct MessageClient {
    server_addr: String,
    token: String,
    node_id: String,
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
    pub tags: Vec<String>,
    pub min_conn: usize,
    pub max_conn: usize,
    pub timeout: Duration,
}

impl MessageClient {
    pub fn new(config: MessageConfig) -> Self {
        let min_conn = config.min_conn.max(1);
        let max_conn = config.max_conn.max(min_conn);

        let client = Self {
            server_addr: config.server_addr,
            token: config.token,
            node_id: config.node_id,
            tags: config.tags,
            connections: Arc::new(Mutex::new(BinaryHeap::new())),
            max_conn,
            min_conn,
            timeout: config.timeout,
        };

        client.init_connections();
        client
    }

    fn init_connections(&self) {
        for _ in 0..self.min_conn {
            self.retry_create_connection();
        }
    }

    fn retry_create_connection(&self) {
        let server_addr = self.server_addr.clone();
        let timeout = self.timeout;
        let connections = self.connections.clone();

        tokio::spawn(async move {
            loop {
                let result = dial_timeout(&server_addr, timeout).await;
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
        });
    }

    pub async fn allocate_connection(
        &self
    ) -> Result<ZenithConnection, Box<dyn std::error::Error + Send + Sync>> {
        let mut conn_pool = self.connections.lock().unwrap();

        if conn_pool.is_empty() && conn_pool.len() < self.max_conn {
            self.retry_create_connection();
        }

        if conn_pool.is_empty() {
            return Err("No available connections".into());
        }

        let conn = conn_pool.pop().unwrap();
        Ok(conn.conn)
    }

    pub async fn free_connection(&self, conn: ZenithConnection) {
        let mut conn_pool = self.connections.lock().unwrap();
        let conn_pool_item = ConnectionPool {
            conn,
            loan_count: 0,
        };
        conn_pool.push(conn_pool_item);
    }
}

impl MessageClient {
    async fn authenticate(
        &self,
        conn: &mut ZenithConnection
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let stmt = LoginStatement::new(
            &self.token,
            self.node_id.clone(),
            self.node_id.clone(),
            false,
            self.tags.clone()
        )?;
        let login_message = Message::new(protocol::MessageType::Login, &stmt);
        let response = match conn.send(login_message).await {
            Ok(response) => response,
            Err(e) => {
                return Err(e);
            }
        };

        if response.header.message_type != MessageType::Login {
            return Err("Authentication failed".into());
        }
        Ok(())
    }
}
