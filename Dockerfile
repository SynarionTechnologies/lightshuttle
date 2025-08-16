FROM rust:1.86-slim AS builder

WORKDIR /app
COPY . .
RUN cargo build --locked --release --manifest-path daemon/Cargo.toml

FROM debian:stable-slim AS dockercli
WORKDIR /docker

ARG DOCKER_VERSION=28.3.3
ARG DOCKER_SHA256=40c16bcf324f354b382d07e845e6a79e3493fc0c09b252dff9e1a46125589bff
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates curl \
    && curl -fsSL "https://download.docker.com/linux/static/stable/x86_64/docker-${DOCKER_VERSION}.tgz" -o docker.tgz \
    && echo "${DOCKER_SHA256}  docker.tgz" | sha256sum -c - \
    && tar -xzf docker.tgz \
    && mv docker/docker /usr/bin/docker \
    && chmod +x /usr/bin/docker \
    && rm -r docker docker.tgz \
    && apt-get purge -y --auto-remove curl \
    && rm -rf /var/lib/apt/lists/*

FROM gcr.io/distroless/cc-debian12:nonroot

WORKDIR /app

COPY --from=builder /app/target/release/lightshuttle_core /usr/local/bin/lightshuttle
COPY --from=dockercli /usr/bin/docker /usr/local/bin/docker
COPY seccomp-profile.json /seccomp.json

USER nonroot

ENV BIND_ADDRESS=0.0.0.0:7878
EXPOSE 7878

CMD ["lightshuttle"]
    
