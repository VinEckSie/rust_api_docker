# 1. Build stage
FROM rust:1.86.0-slim-bullseye as builder

WORKDIR /app

# Copy and build dependencies first
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Now copy full source and rebuild
COPY . .
RUN cargo build --release

# 2. Runtime stage
FROM debian:bullseye-slim

COPY --from=builder /app/target/release/rust_api_docker /usr/local/bin/rust_api_docker


CMD ["rust_api_docker"]
EXPOSE 8080
