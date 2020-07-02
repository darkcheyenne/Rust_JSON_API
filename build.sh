#/bin/bash
echo 🛠 Kompilieren von Rust Binärdate mit Cargo
cargo build --release

echo 🐳 Verpacken von Binärdatei in Docker Image
docker build --pull -t rust_json_api:latest .
