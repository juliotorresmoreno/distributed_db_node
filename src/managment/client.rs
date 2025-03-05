use std::time::Duration;
use futures::StreamExt;
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async,
    tungstenite::protocol::Message,
    MaybeTlsStream,
    WebSocketStream,
};
use url::Url;
use tokio::time::sleep;
use hmac::{ Hmac, Mac };
use sha2::Sha256;
use serde::Deserialize;
use reqwest;

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Deserialize)]
struct NodesResponse {
    connected_masters: Vec<String>,
    connected_slaves: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Client {
    node_id: String,
    cluster_token: String,
    admin_addr: String,
}

impl Client {
    pub fn new(node_id: String, cluster_token: String, admin_addr: String) -> Self {
        return Self {
            node_id,
            cluster_token,
            admin_addr,
        };
    }

    fn generate_token(&self) -> String {
        let mut mac = HmacSha256::new_from_slice(self.cluster_token.as_bytes()).expect(
            "HMAC can take key of any size"
        );
        mac.update(self.node_id.as_bytes());
        return hex::encode(mac.finalize().into_bytes());
    }

    pub async fn connect_to_management(&self) {
        let token = self.generate_token();
        let ws_url = format!(
            "{}/managment/ws/slave?node_id={}&token={}",
            self.admin_addr.replace("http", "ws"),
            self.node_id,
            token
        );

        loop {
            match connect_async(&ws_url).await {
                Ok((ws_stream, _)) => {
                    println!("Connected to management node: {}", self.admin_addr);
                    self.listen(ws_stream).await;
                }
                Err(e) => {
                    eprintln!("Failed to connect: {}. Retrying in 5 seconds...", e);
                    sleep(Duration::from_secs(5)).await;
                }
            }
        }
    }

    async fn listen(&self, mut ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>) {
        while let Some(Ok(msg)) = ws_stream.next().await {
            match msg {
                Message::Text(text) => {
                    println!("Message from management node: {}", text);
                }
                Message::Close(_) => {
                    println!("Connection closed by server.");
                    break;
                }
                _ => {}
            }
        }
    }

    pub async fn get_master_nodes(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let url = format!("{}/managment/nodes", self.admin_addr);
        let response: NodesResponse = reqwest::get(&url).await?.json().await?;
        return Ok(response.connected_masters);
    }
}
