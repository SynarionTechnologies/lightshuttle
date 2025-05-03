# ---- Build stage ----
FROM rust:1.86-slim AS builder

WORKDIR /app
COPY . .

# Build the daemon in release mode
RUN cargo build --release --manifest-path daemon/Cargo.toml

# ---- Runtime stage ----
FROM debian:stable-slim

# Install required Docker CLI
RUN apt-get update && \
    apt-get install -y docker.io ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/lightshuttle_core /usr/local/bin/lightshuttle

ENV BIND_ADDRESS=0.0.0.0:7878

EXPOSE 7878

CMD ["lightshuttle"]
