# 🛡️ Hybrid Deserialization Security Shield (Python & Rust)

<p align="center">
  <img src="https://www.istinye.edu.tr/sites/default/files/isu_logo_tr.png" width="200" alt="İstinye Üniversitesi Logo">
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Language-Rust-orange?style=for-the-badge&logo=rust" alt="Rust">
  <img src="https://img.shields.io/badge/Language-Python-blue?style=for-the-badge&logo=python" alt="Python">
  <img src="https://img.shields.io/badge/Security-Advanced-red?style=for-the-badge" alt="Security">
  <img src="https://img.shields.io/badge/License-MIT-green?style=for-the-badge" alt="License">
</p>

---

## 📑 İçindekiler
* [📖 Proje Hakkında](#-proje-hakkında)
* [🏗️ Mimari Yapı](#-mimari-yapı)
* [🚀 Kurulum ve Çalıştırma](#-kurulum-ve-çalıştırma)
* [🎬 Demo](#-demo)
* [👨‍🏫 Akademik Bilgiler](#-akademik-bilgiler)
* [⚖️ Lisans](#-lisans)

---

## 📖 Proje Hakkında
Bu proje, Python'un `pickle` kütüphanesindeki **Insecure Deserialization** (Güvenli Olmayan Serisizleştirme) zafiyetini (OWASP A08:2021) analiz etmek ve bu zafiyete karşı **Rust** tabanlı hibrit bir savunma mekanizması geliştirmek amacıyla hazırlanmıştır.

### 🔍 Zafiyet Analizi
Python `pickle` modülü, veriyi deserialize ederken kaynağı doğrulamaz. Saldırganlar `__reduce__` metodunu manipüle ederek sistemde **Uzaktan Kod Çalıştırma (RCE)** gerçekleştirebilir. Bu projede, savunma katmanı Python'dan daha güvenli bir bellek yönetimine sahip olan Rust katmanına taşınmıştır.

---

## 🏗️ Mimari Yapı
Proje, "Derinlemesine Savunma" (Defense in Depth) prensibine göre ikiye ayrılmıştır:

1.  **Security Engine (Rust):** Kritik veri doğrulama motoru. `#[serde(deny_unknown_fields)]` kullanarak tanımlanmamış her türlü veriyi (saldırı kodlarını) anında reddeder.
2.  **Secure Server (Python):** Kullanıcı etkileşimini yöneten ama Rust motorundan onay almadan hiçbir veriyi işlemeyen güvenli sunucu katmanı.

---

## 🚀 Kurulum ve Çalıştırma

### Gereksinimler
* Python 3.10+
* Rust & Cargo
* Maturin (`pip install maturin`)

### Adımlar
```bash
# 1. Rust Güvenlik Motorunu Derleyin
cd Security_Engine
maturin develop

# 2. Güvenli Sunucuyu Başlatın
cd ..
python3 secure_server.py


👨‍🏫 Akademik Bilgiler
Üniversite: İstinye Üniversitesi (İSÜ) - Topkapı Kampüsü

Bölüm: Bilişim Güvenliği Teknolojisi (BGT)

Öğrenci: Abdulkadir ERKAN

Ders: Sızma Testi ve Zafiyet Analizi

Danışman: Keyvan Arasteh

⚖️ Lisans
Bu proje MIT Lisansı altında korunmaktadır. Detaylar için LICENSE dosyasına bakınız.




