# Rust builder stage
FROM rust:1.75 as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

# Final runtime stage
FROM python:3.10-slim
WORKDIR /app

# Install python dependencies
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

# Copy everything else
COPY . .

# Copy the compiled rust binary from builder
COPY --from=builder /usr/src/app/target/release/rust_security_project ./rust_security_engine

# Sunucuyu başlat (Örn: vulnerable_server.py)
CMD ["python", "vulnerable_server.py"]
