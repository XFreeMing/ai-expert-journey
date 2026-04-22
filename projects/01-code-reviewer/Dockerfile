FROM lukemathwalker/cargo-chef:latest-rust-1.75-bookworm AS chef
WORKDIR /app

FROM chef AS planner
COPY projects/01-code-reviewer/ ./projects/01-code-reviewer/
COPY shared/ ./shared/
RUN cargo chef prepare --recipe-path recipe.json --manifest-path projects/01-code-reviewer/Cargo.toml

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json --manifest-path projects/01-code-reviewer/Cargo.toml
COPY projects/01-code-reviewer/ ./projects/01-code-reviewer/
COPY shared/ ./shared/
RUN cargo build --release --bin code-reviewer --manifest-path projects/01-code-reviewer/Cargo.toml

FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/code-reviewer /usr/local/bin/
ENTRYPOINT ["code-reviewer"]
