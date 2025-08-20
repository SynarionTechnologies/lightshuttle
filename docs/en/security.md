# Security

LightShuttle communicates with the Docker daemon through a local proxy. Set the `DOCKER_HOST`
environment variable to `http://docker-proxy:2375` so all Docker commands go through the proxy.

To enable authentication on the HTTP API, configure the `JWT_SECRET` environment variable with a
secret of at least 32 characters. Tokens are signed with **HS256** and must include an `exp`
claim. Clients send `Authorization: Bearer <token>`.

Example setup:

```bash
export JWT_SECRET=$(openssl rand -hex 32)
```

Token generation in Rust:

```rust
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::Serialize;

#[derive(Serialize)]
struct Claims { sub: String, exp: usize }

let token = encode(
    &Header::default(),
    &Claims { sub: "demo".into(), exp: 1_700_000_000 },
    &EncodingKey::from_secret(std::env::var("JWT_SECRET")?.as_bytes()),
)?;
```

Running the daemon or CLI as `root` is not supported. If launched as `root`, the process exits
immediately.

The Docker image bundles a seccomp profile at `/seccomp.json`. LightShuttle uses it when
spawning containers to restrict available system calls. Override the profile path by setting the
`SECCOMP_PROFILE` environment variable.
