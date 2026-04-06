use axum::{
    extract::Query,
    response::Html,
    routing::get,
    Router,
};
use serde::Deserialize;
use std::process::Command;
use base64::{Engine as _, engine::general_purpose};

// 1. Zafiyetli Veri Yapısı (JSON buraya map edilecek)
#[derive(Deserialize, Debug)]
struct UserSession {
    username: String,
    task: String, // Saldırganın komut göndereceği yer burası
}

#[tokio::main]
async fn main() {
    // Rota tanımlama: Ana sayfaya gelen istekleri 'index' fonksiyonuna gönder
    let app = Router::new().route("/", get(index));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:5000").await.unwrap();
    println!("🚀 Sunucu ayağa kalktı: http://127.0.0.1:5000");
    println!("Sızma testi için hazır!");
    
    axum::serve(listener, app).await.unwrap();
}

async fn index(Query(params): Query<std::collections::HashMap<String, String>>) -> Html<String> {
    // Eğer URL'de ?session=... parametresi varsa işle
    if let Some(cookie_data) = params.get("session") {
        
        // 2. ZAFİYET NOKTASI: Gelen Base64 verisini sorgusuz sualsiz çözüyoruz
        let decoded_result = general_purpose::STANDARD.decode(cookie_data);
        
        if let Ok(decoded) = decoded_result {
            let session_str = String::from_utf8_lossy(&decoded);
            
            // JSON verisini UserSession yapısına çeviriyoruz (Deserialization)
            let user_result: Result<UserSession, _> = serde_json::from_str(&session_str);

            if let Ok(user) = user_result {
                // 3. EXPLOITATION (Sömürme): 
                // Gelen 'task' içeriğini sistem komutu olarak çalıştırıyoruz (RCE)
                let output = Command::new("sh")
                    .arg("-c")
                    .arg(&user.task)
                    .output();

                return match output {
                    Ok(_) => Html(format!("<h1>Merhaba {}, '{}' görevi başarıyla tetiklendi!</h1>", user.username, user.task)),
                    Err(_) => Html("<h1>Komut çalıştırma hatası!</h1>".to_string()),
                };
            }
        }
        return Html("<h1>Geçersiz session verisi!</h1>".to_string());
    }

    // Parametre yoksa gösterilecek giriş ekranı
    Html("<h1>Vize Projesi: Insecure Deserialization (Rust)</h1><p>Lütfen session parametresini kullanın.</p>".to_string())
}
