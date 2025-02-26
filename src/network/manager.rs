use serde::{ Deserialize, Serialize };
use tokio::{io::AsyncWriteExt, net::TcpStream};
use std::str;

#[derive(Debug, Serialize, Deserialize)]
struct KeyValue {
    key: String,
    value: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    message: Option<String>,
}

pub struct Manager {
    pub address: String,
    pub stream: Option<TcpStream>,
}

impl Manager {
    pub fn new(address: &str) -> Self {
        return Self {
            address: address.to_string(),
            stream: None,
        };
    }

    pub async fn connect(&mut self) -> Result<(), std::io::Error> {
        self.stream = match TcpStream::connect(&self.address).await {
            Ok(stream) => Some(stream),
            Err(e) => return Err(e),
        };

        return Ok(());
    }

    pub async fn send(&mut self, request: &str) -> Result<String, std::io::Error> {
        println!("Enviando solicitud al servidor...");

        let stream: &mut TcpStream = match self.stream.as_mut() {
            Some(stream) => stream,
            None => return Err(std::io::Error::new(std::io::ErrorKind::NotConnected, "No se ha establecido una conexión")),
        };

        stream.write_all(request.as_bytes()).await?;

        return Ok("".to_string());
    }
}
