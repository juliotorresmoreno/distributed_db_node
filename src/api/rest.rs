use actix_web::{ web, App, HttpServer, HttpResponse, HttpRequest, Responder };
use serde::{ Deserialize, Serialize };
use std::sync::{ Arc, Mutex };
use crate::storage::kv_store::KVStore;

#[derive(Debug, Deserialize)]
struct KeyValue {
    key: String,
    value: String,
}

#[derive(Debug, Serialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    message: Option<String>,
}

pub struct RestApi {
    port: u16,
    storage: Arc<Mutex<KVStore>>,
}

impl RestApi {
    pub fn new(port: u16, storage: Arc<Mutex<KVStore>>) -> Self {
        Self { port, storage }
    }

    pub async fn run(&self) -> std::io::Result<()> {
        let storage = self.storage.clone();
        let port = self.port;

        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(storage.clone()))
                .route("/kv/{key}", web::get().to(get_key))
                .route("/kv", web::post().to(set_key))
                .route("/kv/{key}", web::delete().to(delete_key))
        })
            .bind(("0.0.0.0", port))?
            .run().await
    }
}

async fn get_key(req: HttpRequest, storage: web::Data<Arc<Mutex<KVStore>>>) -> impl Responder {
    let key = req.match_info().get("key").unwrap().to_string();
    let store = storage.lock().unwrap();
    let value = store.get(&key);

    match value {
        Some(v) =>
            HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(v),
                message: None,
            }),
        _none =>
            HttpResponse::NotFound().json(ApiResponse::<String> {
                success: false,
                data: None,
                message: Some("Key not found".to_string()),
            }),
    }
}

async fn set_key(
    kv: web::Json<KeyValue>,
    storage: web::Data<Arc<Mutex<KVStore>>>
) -> impl Responder {
    let mut store = storage.lock().unwrap();
    store.set(kv.key.clone(), kv.value.clone());

    return HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        message: Some("Key set successfully".to_string()),
    });
}

async fn delete_key(req: HttpRequest, storage: web::Data<Arc<Mutex<KVStore>>>) -> impl Responder {
    let key = req.match_info().get("key").unwrap().to_string();
    let mut store = storage.lock().unwrap();
    let deleted = store.delete(&key);

    if deleted {
        return HttpResponse::Ok().json(ApiResponse::<Option<()>> {
            success: true,
            data: None,
            message: Some("Key deleted successfully".to_string()),
        });
    }
    
    return HttpResponse::NotFound().json(ApiResponse::<Option<()>> {
        success: false,
        data: None,
        message: Some("Key not found".to_string()),
    });
}
