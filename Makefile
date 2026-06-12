.PHONY: up down build dev check app web server

up:
	docker compose -f docker/docker-compose.yml up -d

down:
	docker compose -f docker/docker-compose.yml down

build:
	docker image prune -f
	docker compose -f docker/docker-compose.yml build

dev:
	docker compose -f docker/docker-compose.yml --profile dev up server-dev

check:
	cargo fmt --all --check
	cargo clippy --all-targets -- -D warnings
	cargo clippy -p neon-ante-games --all-targets --features serde -- -D warnings
	cargo clippy -p neon-ante-web --target wasm32-unknown-unknown -- -D warnings
	cargo test

app:
	cargo run -p neon-ante-app

web:
	cd web && trunk serve --open

server:
	cd web && trunk build && cd .. && STATIC_DIR=web/dist cargo run -p neon-ante-server
