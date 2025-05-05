![Build](https://github.com/LightShuttle/lightshuttle/actions/workflows/ci.yml/badge.svg)
![Build](https://github.com/LightShuttle/lightshuttle/actions/workflows/docker-publish.yml/badge.svg)
![Docker Image](https://img.shields.io/docker/pulls/synarion/lightshuttle?style=flat-square)

# LightShuttle

🚀 LightShuttle is a lightweight, fast, and self-hostable orchestrator for containerized applications, designed as a simple alternative to Kubernetes.

---

## Features

- ⚡ Ultra-fast API based on [Axum](https://github.com/tokio-rs/axum)
- 🐳 Direct control over Docker CLI (no daemon inside)
- 🔥 Open-source, fully self-hostable
- 🛠 Simple REST API (no GraphQL)
- 📈 Metrics ready for Prometheus
- 🧹 Designed for developers: deploy faster, debug easier

---

## Architecture

- **Daemon** (`daemon/`): the core server handling API requests and container orchestration
- **CLI** (`cli/`): command-line tool (WIP)
- **Dashboard** (`dashboard/`): web UI (planned)

---

## Requirements

- Rust (>= 1.76)
- Docker installed and accessible (`docker` CLI must work)
- Linux recommended (tested on Debian 12)

---

## Local development

```bash
# Clone the repo
git clone https://github.com/LightShuttle/lightshuttle.git
cd lightshuttle

# Install dependencies
cargo install cargo-make

# Build and test
make
```

---

## Docker deployment

You can run LightShuttle directly with Docker:

```bash
docker run -d \
  -p 7878:7878 \
  -e BIND_ADDRESS=0.0.0.0:7878 \
  -v /var/run/docker.sock:/var/run/docker.sock \
  synarion/lightshuttle:latest
```
Or with Docker Compose :

```bash
docker-compose up -d
```

---

## Running the Daemon

```bash
# Build
make

# Launch
cargo run --bin lightshuttle_core
```

By default, the API will be available on [http://127.0.0.1:7878](http://127.0.0.1:7878).

You can override the default address by setting the `BIND_ADDRESS` environment variable:

```bash
BIND_ADDRESS=0.0.0.0:7878 cargo run --bin lightshuttle_core
```

---

## Roadmap

- [x] Basic container lifecycle (create, list, delete, logs)
- [ ] Start/Stop containers
- [ ] Search containers
- [ ] Labels support
- [ ] Update/Recreate containers
- [ ] Full error refinement
- [ ] CLI client (`lightshuttle-cli`)
- [ ] Dashboard web UI
- [ ] Authentication & RBAC
- [ ] Templates system (Helm-light)

---

## License

LightShuttle is licensed under the GNU Affero General Public License v3.0 (AGPLv3).  
See [LICENSE](LICENSE) for more details.

---

## Website

Official website: [https://www.getlightshuttle.com](https://www.getlightshuttle.com)

---

## Credits

Developed by **Pierrick FONQUERNE**.
