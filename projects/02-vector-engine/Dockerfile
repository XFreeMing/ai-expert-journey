FROM lukemathwalker/cargo-chef:latest-rust-1.75-bookworm AS chef
WORKDIR /app

FROM chef AS planner
COPY projects/02-vector-engine/ ./projects/02-vector-engine/
COPY shared/ ./shared/
RUN cargo chef prepare --recipe-path recipe.json --manifest-path projects/02-vector-engine/Cargo.toml

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json --manifest-path projects/02-vector-engine/Cargo.toml
COPY projects/02-vector-engine/ ./projects/02-vector-engine/
COPY shared/ ./shared/
RUN cargo build --release --bin vector-engine --manifest-path projects/02-vector-engine/Cargo.toml

FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/vector-engine /usr/local/bin/
ENTRYPOINT ["vector-engine"]
