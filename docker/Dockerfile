FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /neon-ante
RUN rustup target add wasm32-unknown-unknown
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall -y trunk

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /neon-ante/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
RUN cargo chef cook --release --target wasm32-unknown-unknown -p neon-ante-web --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin neon-ante-server

WORKDIR /neon-ante/web
RUN trunk build --release

FROM gcr.io/distroless/cc-debian13:nonroot AS runtime
WORKDIR /neon-ante
COPY --from=builder /neon-ante/target/release/neon-ante-server /usr/local/bin/server
COPY --from=builder /neon-ante/web/dist /neon-ante/static
CMD ["/usr/local/bin/server"]
