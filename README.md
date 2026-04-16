# 🛡️ Hybrid Deserialization Security Shield (Python & Rust)

 
 <img width="211" height="80" alt="image" src="https://github.com/user-attachments/assets/3be9d094-f4e5-405d-9216-2e2a4d42e4f5" />


[![Language - Rust](https://img.shields.io/badge/Language-Rust-orange?logo=rust)](https://www.rust-lang.org)
[![Language - Python](https://img.shields.io/badge/Language-Python-blue?logo=python)](https://www.python.org)
[![Security - Mitigation](https://img.shields.io/badge/Security-Mitigation-red)](https://owasp.org)

---

## 📑 Table of Contents
1. [📖 About the Project](#-about-the-project)
2. [🏗️ Architectural Design](#-architectural-design)
3. [🚀 Installation & Execution](#-installation--execution)
4. [✅ Test and Verification](#-test-and-verification)
5. [🎬 Demo](#-demo)
6. [👨‍🏫 Academic Information](#-academic-information)
7. [⚖️ License](#-license)

---

## 📖 About the Project
This project was developed to analyze the **Insecure Deserialization (OWASP A08:2021)** vulnerability within Python's `pickle` library and to implement a **Rust-based hybrid defense mechanism** against it.

### 🔍 Vulnerability Analysis
The Python `pickle` module does not verify the source of the data during deserialization. Attackers can manipulate the `__reduce__` method to perform **Remote Code Execution (RCE)**. In this project, the defense layer is offloaded to a **Rust** environment, which offers superior memory management and safety compared to Python.

---

## 🏗️ Architectural Design
The project is designed following the **"Defense in Depth"** principle, consisting of two main layers:

* **Security Engine (Rust):** The critical data validation core. By utilizing strict type checking and memory-safe logic, it instantly rejects undefined data or malicious payloads (attack vectors).
* **Secure Server (Python):** Manages user interaction but refuses to process any data without explicit approval and validation from the Rust security engine.

---

## 🚀 Installation & Execution

### Prerequisites
* Python 3.10+
* Rust & Cargo
* Maturin (`pip install maturin`)

### Steps
```bash
# 1. Compile the Rust Security Engine
cd Security_Engine
maturin develop

# 2. Start the Secure Server
cd ..
python3 secure_server.py

✅ Test and Verification
The security efficacy of this project has been verified through a multi-stage testing process:

Vulnerability Simulation: Using payload_gen.py, a malicious pickle payload was generated targeting an RCE vector.

Mitigation Testing: The secure_server.py was tested against the generated payload. The Rust Security Engine successfully intercepted the unauthorized call and blocked execution.

Clean Data Pass-through: Verified that legitimate serialized data is processed with zero latency, ensuring security does not hinder performance.

Error Handling: Tested with corrupted and null bytes; the engine gracefully rejected input without system crashes.

🎬 Demo
A comprehensive demonstration of the build process, attack simulation, and successful mitigation is integrated below.

[!IMPORTANT]
View the Full Demo Here: ▶️ Project Demo Video

The demo includes:

Compilation and Setup of the Rust Security Engine.

Real-time attack detection and blocking logic.

System file structure and technical architecture walkthrough.



👨‍🏫 Academic Information

Field Details
University İstinye University (İSÜ) - Topkapı Campus
Department Cybersecurity Technology (BGT)
Student Abdulkadir ERKAN
Course Penetration Testing and Vulnerability Analysis
Advisor Keyvan Arasteh