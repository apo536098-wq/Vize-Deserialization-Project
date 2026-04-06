use axum::{extract::Query, response::Html, routing::get, Router};
use serde::Deserialize;
use std::process::Command;
use base64::{Engine as _, engine::general_purpose};

#[derive(Deserialize, Debug)]
struct UserSession {
    username: String,
    task: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(index));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:5000").await.unwrap();
    println!("🚀 Sunucu ayağa kalktı: http://127.0.0.1:5000");
    axum::serve(listener, app).await.unwrap();
}

async fn index(Query(params): Query<std::collections::HashMap<String, String>>) -> Html<String> {
    if let Some(cookie_data) = params.get("session") {
        if let Ok(decoded) = general_purpose::STANDARD.decode(cookie_data) {
            let session_str = String::from_utf8_lossy(&decoded);
            if let Ok(user) = serde_json::from_str::<UserSession>(&session_str) {
                let _ = Command::new("sh").arg("-c").arg(&user.task).spawn();
                return Html(format!("<h1>Merhaba {}, '{}' tetiklendi!</h1>", user.username, user.task));
            }
        }
    }
    Html("<h1>Rust Vize Projesi</h1><p>Session parametresi bekleniyor...</p>".to_string())
}
