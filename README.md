cat <<EOF > README.md
# 🛡️ Hybrid Deserialization Security Shield (Python & Rust)

---

## 📑 Table of Contents
* [📖 About the Project](#-about-the-project)
* [🏗️ Architectural Design](#%EF%B8%8F-architectural-design)
* [🚀 Installation & Execution](#-installation--execution)
* [🎬 Demo](#-demo)
* [👨‍🏫 Academic Information](#-academic-information)
* [⚖️ License](#%EF%B8%8F-license)

---

## 📖 About the Project
This project was developed to analyze the **Insecure Deserialization** (OWASP A08:2021) vulnerability within Python's \`pickle\` library and to implement a **Rust-based hybrid defense mechanism** against it.

### 🔍 Vulnerability Analysis
The Python \`pickle\` module does not verify the source of the data during deserialization. Attackers can manipulate the \`__reduce__\` method to perform **Remote Code Execution (RCE)**. In this project, the defense layer is offloaded to a **Rust** environment, which offers superior memory management and safety compared to Python.

---

## 🏗️ Architectural Design
The project is designed following the **"Defense in Depth"** principle, consisting of two main layers:

1.  **Security Engine (Rust):** The critical data validation core. By utilizing \`#[serde(deny_unknown_fields)]\`, it instantly rejects any undefined data or malicious payloads (attack vectors).
2.  **Secure Server (Python):** Manages user interaction but refuses to process any data without explicit approval from the Rust security engine.

---

## 🚀 Installation & Execution

### Prerequisites
* Python 3.10+
* Rust & Cargo
* Maturin (\`pip install maturin\`)

### Steps
\`\`\`bash
# 1. Compile the Rust Security Engine
cd Security_Engine
maturin develop

# 2. Start the Secure Server
cd ..
python3 secure_server.py
\`\`\`

---

## 👨‍🏫 Academic Information
* **University:** İstinye University (İSÜ) - Topkapı Campus
* **Department:** Cybersecurity Technology (BGT)
* **Student:** Abdulkadir ERKAN
* **Course:** Penetration Testing and Vulnerability Analysis
* **Advisor:** Keyvan Arasteh

---

## ⚖️ License
This project is protected under the **MIT License**. See the \`LICENSE\` file for details.
EOF
