import rust_security_core
import json

print("--- TEST 1: GÜVENLİ VERİ GİRİŞİ ---")
# Rust'ın beklediği tam yapı: username, user_id, role
safe_data = json.dumps({"username": "Kadir", "user_id": 1, "role": "admin"})

try:
    result = rust_security_core.validate_and_process(safe_data)
    print("Rust Çıktısı:", result)
except Exception as e:
    print("Hata:", e)

print("\n" + "="*40 + "\n")

print("--- TEST 2: SALDIRI GİRİŞİMİ (Insecure Deserialization) ---")
# Saldırgan araya 'attack' diye fazladan bir anahtar ve zararlı komut sokuşturuyor
evil_data = json.dumps({
    "username": "Hacker", 
    "user_id": 0, 
    "role": "guest",
    "attack": "os.system('rm -rf /')" 
})

try:
    # Rust burada 'attack' alanını görünce "Benim kalıbımda bu yok!" diyecek
    result = rust_security_core.validate_and_process(evil_data)
    print("Sonuç:", result)
except ValueError as e:
    print("GÜVENLİK MOTORU (RUST) SALDIRIYI ENGELLEDİ!")
    print("Hata Mesajı:", e)
