# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),  
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
