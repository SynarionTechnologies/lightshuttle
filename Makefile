# Makefile

.PHONY: all fmt clippy build run test ci docker-up docker-down

all: fmt clippy build

fmt:
	cargo fmt --all

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

build:
	cargo build

run:
	cargo run --bin lightshuttle_core

test:
	cargo test

ci:
	make fmt
	make clippy
	make test

docker-up:
	docker-compose up -d --build

docker-down:
	docker-compose down
