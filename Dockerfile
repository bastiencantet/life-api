FROM lukemathwalker/cargo-chef:latest-rust-1.80 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --bin life-api --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin life-api

# We do not need the Rust toolchain to run the binary!
FROM debian:12.5-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/life-api /usr/local/bin
RUN apt-get update && apt-get install -y libssl-dev ca-certificates
ENTRYPOINT ["/usr/local/bin/life-api"]