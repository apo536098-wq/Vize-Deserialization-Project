use axum::{routing::get, Router, response::Html};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

// GÜVENLİK KALIBI: Sadece bu alanlara izin var.
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct UserSession {
    username: String,
    user_id: i32,
    role: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(index));

    let addr = SocketAddr::from(([127, 0, 0, 1], 5000));
    println!("🛡️ İstinye Üniversitesi Güvenli Sunucu: http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> Html<&'static str> {
    Html("<h1>🛡️ Rust Güvenli Sunucu Aktif</h1><p>Insecure Deserialization bu sistemde imkansızdır!</p>")
}
