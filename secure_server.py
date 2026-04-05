from flask import Flask, request, make_response
import rust_security_core  # Senin yazdığın o güçlü Rust modülü!
import base64
import json

app = Flask(__name__)

@app.route('/')
def index():
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
