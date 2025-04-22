# LightShuttle

> Ultra-fast and developer-friendly orchestrator for containerized applications. Built in Rust.

LightShuttle is a next-generation orchestrator designed to simplify how developers deploy, debug, and manage containerized applications — without the bloat.

## 🚀 Why LightShuttle?

- ⚡️ Blazing fast (written in Rust)
- 🧠 Developer-first UX (CLI + Dashboard)
- 🧰 Simple deployment, clean logs, instant debug
- 🔌 API-first architecture
- 📦 Lightweight by design

---

## 📦 Project Structure

This repository is a [Rust workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) containing multiple components of the LightShuttle ecosystem:

```bash
lightshuttle/
├── core/         # The orchestrator engine (API + runtime)
├── cli/          # Command-line interface to control the orchestrator
├── dashboard/    # Web-based UI
```

---

## 🛠️ Getting Started

### Prerequisites

- [Rust](https://rust-lang.org/tools/install) (>= 1.75)
- [Docker](https://www.docker.com/) (for development/testing)

### Clone

```bash
git clone https://github.com/LightShuttle/lightshuttle.git
cd lightshuttle
```

### Build
```bash
cargo build --workspace
```

### Run the orchestrator

```bash
cargo run -p lightshuttle_core
```

### Use the CLI

```bash
cargo run -p lightshuttle_cli -- help
```

---

## 🧪 Tests

```bash
cargo test --workspace
```

---

## 🌐 Dashboard

The dashboard is not yet implemented — but will eventually allow you to:

- Visualize deployments and services
- Inspect logs and metrics
- Trigger builds and rollbacks
- Monitor system health

---

## 📄 License

Licensed under the [AGPL-3.0 license](LICENSE).

---

## 🙌 Contributing

Coming soon. Stay tuned.

---
