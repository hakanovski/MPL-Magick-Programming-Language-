FROM rust:1.80-slim AS builder

WORKDIR /usr/src/app
# Create empty projects for caching
RUN USER=root cargo new mpl_compiler
WORKDIR /usr/src/app/mpl_compiler
COPY mpl_compiler/Cargo.toml .

# Cache dependencies
RUN cargo build --release || true

# Build the real binary
COPY mpl_compiler/src ./src
# Update mtime so cargo rebuilds
RUN touch src/main.rs && cargo build --release

# Final runtime image
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates curl && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /usr/src/app/mpl_compiler/target/release/mpl_compiler /usr/local/bin/

ENV RUST_LOG=info
EXPOSE 3690
CMD ["mpl_compiler", "sentinel"]
