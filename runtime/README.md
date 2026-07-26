# runtime

This directory holds the runtime manager that will be responsible for starting/stopping server processes, streaming logs, exposing a local API for the Flutter UI, and performing tasks like UPnP port mapping and backups.

Recommendation (default):
- Language: Rust (small static binaries, good cross-compilation, memory safe).
- API: local HTTP + WebSocket for log streaming and console I/O.
- Behavior: download server artifacts on demand, store per-server data under app data directory, and run processes under user permissions.

Stubs:
- Implement a simple command-line binary that supports:
  - `runtime create --name NAME --engine paper --version 1.20.2`
  - `runtime start --id ID`
  - `runtime stop --id ID`
  - `runtime status --id ID`
  - `runtime serve --socket /tmp/mineservers.sock` to start an HTTP+WebSocket control server

Next tasks:
1. Create Rust project using `cargo init --bin runtime`.
2. Add an HTTP/WebSocket crate (e.g., axum + tokio-tungstenite) and implement a minimal control API.
3. Implement process management using std::process and proper shutdown handling.
4. Add cross-compilation build scripts for Windows/macOS (x64 + arm64) and Linux.
