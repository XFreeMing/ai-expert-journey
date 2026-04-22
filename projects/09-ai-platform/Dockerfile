FROM lukemathwalker/cargo-chef:latest-rust-1.75-bookworm AS chef
WORKDIR /app

FROM chef AS planner
COPY projects/09-ai-platform/ ./projects/09-ai-platform/
COPY shared/ ./shared/
RUN cargo chef prepare --recipe-path recipe.json --manifest-path projects/09-ai-platform/Cargo.toml

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json --manifest-path projects/09-ai-platform/Cargo.toml
COPY projects/09-ai-platform/ ./projects/09-ai-platform/
COPY shared/ ./shared/
RUN cargo build --release --bin ai-platform-gateway --manifest-path projects/09-ai-platform/Cargo.toml

FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/ai-platform-gateway /usr/local/bin/
EXPOSE 8000
ENTRYPOINT ["ai-platform-gateway"]
