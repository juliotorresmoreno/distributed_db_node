use std::sync::Arc;
use std::time::{ Duration, SystemTime };
use futures::{ SinkExt, StreamExt };
use tokio::net::TcpStream;
use tokio::time::sleep;
use tokio_tungstenite::{
    connect_async,
    tungstenite::protocol::Message,
    MaybeTlsStream,
    WebSocketStream,
};
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use hmac::{ Hmac, Mac };
use sha2::Sha256;
use serde::Serialize;
use serde_json::Value;
use chrono;
use base64::{ self, Engine, engine::general_purpose::STANDARD };

type HmacSha256 = Hmac<Sha256>;
type EventHandler = Arc<dyn Fn(Vec<String>) + Send + Sync>;

#[derive(Debug, Serialize)]
struct RegisterMessage {
    action: String,
    node_id: String,
    payload: String, // Ahora el payload es una string en Base64
}

#[derive(Clone)]
pub struct Client {
    node_id: String,
    cluster_token: String,
    admin_addr: String,
    url: String,
    event_handler: Option<EventHandler>,
}

impl Client {
    pub fn new(node_id: String, cluster_token: String, admin_addr: String, url: String) -> Self {
        Self {
            node_id,
            cluster_token,
            admin_addr,
            url,
            event_handler: None,
        }
    }

    pub fn with_event_handler(mut self, handler: EventHandler) -> Self {
        self.event_handler = Some(handler);
        self
    }

    fn get_current_date() -> String {
        let now = SystemTime::now();
        let datetime: chrono::DateTime<chrono::Utc> = now.into();
        datetime.to_rfc3339()
    }

    fn generate_token(&self, date: &str) -> String {
        let mut mac = HmacSha256::new_from_slice(self.cluster_token.as_bytes()).expect(
            "HMAC can take key of any size"
        );
        let data = format!("{}|{}", self.node_id, date);
        mac.update(data.as_bytes());
        hex::encode(mac.finalize().into_bytes())
    }

    pub async fn connect_to_management(&self) {
        loop {
            let date = Self::get_current_date();
            let token = self.generate_token(&date);

            let ws_url = format!(
                "{}/managment/ws/slave?node_id={}",
                self.admin_addr.replace("http", "ws"),
                self.node_id
            );

            let mut request = match ws_url.clone().into_client_request() {
                Ok(req) => req,
                Err(e) => {
                    eprintln!("Invalid WebSocket URL: {}", e);
                    sleep(Duration::from_secs(5)).await;
                    continue;
                }
            };

            request
                .headers_mut()
                .insert("Authorization", format!("Bearer {}", token).parse().unwrap());
            request.headers_mut().insert("Date", date.parse().unwrap());

            match connect_async(request).await {
                Ok((mut ws_stream, _)) => {
                    println!("Connected to management node: {}", ws_url);
                    self.register(&mut ws_stream).await;
                    self.listen(ws_stream).await;
                }
                Err(e) => {
                    eprintln!("Failed to connect: {}. Retrying in 5 seconds...", e);
                    sleep(Duration::from_secs(5)).await;
                }
            }
        }
    }

    async fn register(&self, ws_stream: &mut WebSocketStream<MaybeTlsStream<TcpStream>>) {
        let payload_json = serde_json::json!({ "url": self.url });

        // Serializar a JSON y luego a Base64
        match serde_json::to_vec(&payload_json) {
            Ok(payload_bytes) => {
                let payload_base64 = STANDARD.encode(payload_bytes);

                let register_msg = RegisterMessage {
                    action: "register".to_string(),
                    node_id: self.node_id.clone(),
                    payload: payload_base64, // Enviamos en Base64
                };

                let message_text = serde_json::to_string(&register_msg).unwrap();
                ws_stream.send(Message::Text(message_text.into())).await.unwrap();
            }
            Err(e) => {
                eprintln!("Failed to serialize register message: {}", e);
            }
        }
    }

    async fn listen(&self, mut ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>) {
        while let Some(Ok(msg)) = ws_stream.next().await {
            match msg {
                Message::Text(text) => {
                    if let Ok(event) = serde_json::from_str::<Value>(&text) {
                        if let Some(action) = event.get("action").and_then(|a| a.as_str()) {
                            match action {
                                "master_list" => {
                                    if
                                        let Some(masters) = event
                                            .get("payload")
                                            .and_then(|p| p.get("masters"))
                                            .and_then(|m| m.as_array())
                                    {
                                        let master_urls: Vec<String> = masters
                                            .iter()
                                            .filter_map(|m| m.as_str().map(|s| s.to_string()))
                                            .collect();

                                        println!("Received master list: {:?}", master_urls);

                                        if let Some(handler) = &self.event_handler {
                                            handler(master_urls);
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
                Message::Close(_) => {
                    println!("Connection closed by server.");
                    break;
                }
                _ => {}
            }
        }
    }
}
