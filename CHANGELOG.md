# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),  
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.2.0] – 2025-05-05

### Added
- 🚀 Support for Docker volumes via `volumes: ["/host:/container"]`
- ♻️ Container recreation (`POST /apps/{name}/recreate`) preserving config (env, ports, labels, volumes, restart policy)
- 🟢 Start / Stop container endpoints (`POST /apps/{name}/start` and `.../stop`)
- 🔍 Search support in `GET /apps?search=...`
- 🏷 Container labels via `labels: { "key": "value" }`
- 🌱 Restart policy support (`always`, `on-failure`, etc.)
- 🧪 Extensive Docker-based integration tests with `DOCKER_TEST=1`

### Changed
- 🧱 Refactored `create_and_run_container` into `ContainerConfig` struct (breaking change for internal API)
- 🧹 Codebase cleaned for Clippy compliance (`-D warnings`)
- 📦 CI enforces fmt + clippy + test

### Fixed
- 🔒 Invalid volume format now correctly returns `400 Bad Request`
- ✅ Recreate now properly handles missing port bindings or empty labels/envs

### Removed
- ❌ Legacy signature for `create_and_run_container` (replaced by struct-based API)

---

## [0.1.0-alpha] - 2025-04-28

### Added
- First public alpha release of LightShuttle
- REST API to manage containerized applications
  - `POST /apps` - Create and launch a container
  - `GET /apps` - List running containers (paginated)
  - `GET /apps/:name` - Retrieve details of a container
  - `GET /apps/:name/logs` - Retrieve container logs
  - `DELETE /apps/:name` - Remove a running container
- Error handling system
- Docker CLI interaction layer
- Makefile automation (`make`, `make run`, `make docker-up`, etc.)
- GitHub templates for Issues, Pull Requests, Security
- Initial security policy (SECURITY.md)

### Notes
- This version is experimental and for development/testing purposes only.
- APIs may still change before reaching a stable v1.0.0 release.

---
