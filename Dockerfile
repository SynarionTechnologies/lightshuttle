# ---- Build stage ----
    FROM rust:1.86-slim AS builder

    WORKDIR /app
    COPY . .
    RUN cargo build --release --manifest-path daemon/Cargo.toml
    
    # ---- Runtime stage ----
    FROM debian:stable-slim
    
    WORKDIR /app
    
    # Install Docker CLI only (no daemon, no apt)
    RUN apt-get update && apt-get install -y curl ca-certificates && \
        curl -fsSL https://download.docker.com/linux/static/stable/x86_64/docker-25.0.3.tgz \
        | tar xz --strip-components=1 -C /usr/local/bin docker/docker && \
        apt-get remove -y curl && apt-get autoremove -y && rm -rf /var/lib/apt/lists/*
    
    COPY --from=builder /app/target/release/lightshuttle_core /usr/local/bin/lightshuttle
    
    # Non-root user (compliance Docker Scout)
    RUN useradd -m -u 1000 lightshuttle
    USER lightshuttle
    
    ENV BIND_ADDRESS=0.0.0.0:7878
    EXPOSE 7878
    
    CMD ["lightshuttle"]
    