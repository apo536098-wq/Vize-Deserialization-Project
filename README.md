# 🛡️ Insecure Deserialization & Rust-Powered Mitigation (PoC)

This project demonstrates the **Insecure Deserialization** vulnerability within Python's `pickle` library and showcases a modern **Mitigation** strategy using **Rust** to build a high-security hybrid architecture.

---

## 🔍 Vulnerability Analysis

The application stores user session data on the client-side within a cookie, encoded in Base64 as a `pickle` object. Since `pickle.loads()` does not verify the source or the content of the data during deserialization, it allows an attacker to inject malicious commands, leading to **Remote Code Execution (RCE)**.

| Component | Detail |
| :--- | :--- |
| **Vulnerability Type** | Insecure Deserialization (OWASP A08:2021) |
| **Critical Function** | `pickle.loads()` |
| **Attack Vector** | `__reduce__` method manipulation |
| **Impact** | Remote Code Execution (RCE) |



---

## 🛠️ Project Structure & Content

The project consists of two main sections: the **Attack Scenario** and the **Modern Defense Layer**.

* **`vulnerable_server.py`**: A vulnerable Python server that blindly accepts objects, leaving it open to RCE attacks.
* **`payload_gen.py`**: A Python script designed to generate a malicious payload that creates a `HACKED.txt` file on the system.
* **`Security_Engine/`**: The core of the project; a high-security validation engine written in **Rust**.
* **`secure_server.py`**: A secure server that utilizes the Rust engine as a "Security Gateway" to instantly block intrusion attempts.

---

## 🚀 Mitigation Strategy: Rust Hybrid Security Layer

The most unique aspect of this project is moving the defense from the application layer (Python) to the **Memory Safety** layer provided by **Rust**:

1.  **Strict Schema Enforcement:** Using Rust's `#[serde(deny_unknown_fields)]` rule, any undefined fields (attack codes, etc.) are rejected immediately.
2.  **Zero Trust Approach:** Data is treated as "potentially malicious" until it is strictly validated by the Rust engine.
3.  **Type Safety:** Unlike Python’s dynamic and flexible nature, Rust enforces rigid data structures, making payload manipulation nearly impossible.



---

## ⚙️ Setup and Execution

### 1. Requirements
* Python 3.10+
* Rust & Cargo
* Maturin (`pip install maturin`)

### 2. Compiling the Rust Security Engine
```bash
cd Security_Engine
maturin develop

3. Testing the Secure Server
Bash
python3 secure_server.py
Note: When an attacker sends a malicious payload, a "SECURITY BREACH" warning will be displayed in the terminal, and the system will remain protected.


👨‍💻 Author
Name: ABDULKADİR ERKAN

Department: Information Security Technology (Bilişim Güvenliği Teknolojisi)

University: Istinye University (İSÜ) - Topkapı Campus
