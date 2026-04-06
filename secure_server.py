from flask import Flask, request, make_response
<<<<<<< HEAD
import rust_security_core  # Senin yazdığın o güçlü Rust modülü!
=======
import rust_deserialization_project as rust_security
>>>>>>> fa8598c (Rust-Python Hibrit Güvenlik Katmanı: Vize Projesi Tamamlandı)
import base64
import json

app = Flask(__name__)

@app.route('/')
def index():
<<<<<<< HEAD
    # Cookie'den veriyi al
    session_cookie = request.cookies.get('user_session')
    
    if not session_cookie:
        return "Lütfen geçerli bir session cookie ayarlayın."

    try:
        # 1. ADIM: Base64 Çöz (Zararlı veri burada saklı olabilir)
        decoded_data = base64.b64decode(session_cookie).decode('utf-8')
        
        # 2. ADIM: RUST GÜVENLİK MOTORUNA GÖNDER (Kritik Nokta!)
        # Rust, içinde 'attack' veya bilinmeyen alan olan veriyi burada BLOKLAR.
        valid_user = rust_security_core.validate_and_process(decoded_data)
        
        return f"<h1>🛡️ GÜVENLİ ERİŞİM</h1><p>{valid_user}</p>"
        
    except Exception as e:
        # Saldırı girişimi yakalandığında burası çalışır
        return f"<h1>🚨 GÜVENLİK ENGELİ!</h1><p>Zararlı veri tespit edildi ve Rust motoru tarafından durduruldu: {e}</p>", 403

if __name__ == '__main__':
    app.run(debug=True, port=5001) # Farklı portta çalıştıralım
=======
    session_cookie = request.cookies.get('user_session')
    if not session_cookie:
        return "<h1>🛡️ Durum: Beklemede</h1><p>Lütfen geçerli bir session cookie ayarlayın.</p>"

    try:
        decoded_data = base64.b64decode(session_cookie).decode('utf-8')
        # Rust motorun burada devreye giriyor
        valid_user = rust_security.validate_and_process(decoded_data)
        return f"<h1>🛡️ GÜVENLİ ERİŞİM</h1><p>Giriş Yapan Kullanıcı: {valid_user}</p>"
    except Exception as e:
        return f"<h1>🚨 GÜVENLİK ENGELİ!</h1><p>Rust motoru zararlı veri tespit etti: {e}</p>", 403

if __name__ == '__main__':
    print("--- Flask Sunucusu Başlatılıyor (Port: 5001) ---")
    app.run(debug=True, port=5001)
>>>>>>> fa8598c (Rust-Python Hibrit Güvenlik Katmanı: Vize Projesi Tamamlandı)
