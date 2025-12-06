use crate::config::ProjectConfig;

pub fn generate(_config: &ProjectConfig) -> &'static str {
    r#"# Build stage
FROM rust:1.75 as builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY migrations ./migrations

RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/* /app/
COPY --from=builder /app/migrations ./migrations

ENV RUST_LOG=info

EXPOSE 3000

CMD ["./app"]
"#
}
