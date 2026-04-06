# System Architecture

This project integrates the flexibility of **Python** with the memory safety and performance of **Rust**. 

### Working Principle:
1. **Detection Layer (Python):** Potential malicious data or untrusted serialized objects are first captured at the application level.
2. **Security Engine (Rust):** The data is then passed to the Rust-powered engine for deep inspection.
3. **Validation & Mitigation:** The Rust layer validates the integrity of the data and performs memory-safe deserialization, effectively mitigating **Insecure Deserialization** attacks before they reach the main system logic.

By offloading critical security checks to Rust, the project ensures high performance and prevents common memory-related vulnerabilities.
