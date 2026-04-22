FROM lukemathwalker/cargo-chef:latest-rust-1.75-bookworm AS chef
WORKDIR /app

FROM chef AS planner
COPY projects/06-llm-inference/ ./projects/06-llm-inference/
COPY shared/ ./shared/
RUN cargo chef prepare --recipe-path recipe.json --manifest-path projects/06-llm-inference/Cargo.toml

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json --manifest-path projects/06-llm-inference/Cargo.toml
COPY projects/06-llm-inference/ ./projects/06-llm-inference/
COPY shared/ ./shared/
RUN cargo build --release --bin llm-inference-tools --manifest-path projects/06-llm-inference/Cargo.toml

FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/llm-inference-tools /usr/local/bin/
ENTRYPOINT ["llm-inference-tools"]
