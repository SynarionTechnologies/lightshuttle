    FROM rust:1.86-slim as builder

    WORKDIR /app
    COPY . .
    RUN cargo build --release --manifest-path daemon/Cargo.toml
    
    FROM busybox:1.37 as dockercli
    WORKDIR /docker
    
    ADD https://download.docker.com/linux/static/stable/x86_64/docker-25.0.3.tgz docker.tgz
    RUN tar -xzf docker.tgz && mv docker/docker /usr/bin/docker && chmod +x /usr/bin/docker
    
    FROM debian:stable-slim
    
    WORKDIR /app
    
    COPY --from=builder /app/target/release/lightshuttle_core /usr/local/bin/lightshuttle
    COPY --from=dockercli /usr/bin/docker /usr/local/bin/docker
    
    RUN useradd -m -u 1000 lightshuttle
    USER lightshuttle
    
    ENV BIND_ADDRESS=0.0.0.0:7878
    EXPOSE 7878
    
    CMD ["lightshuttle"]
    