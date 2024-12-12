
# Build stage
FROM rust:latest as builder

WORKDIR /app

# Accept the build argument
ARG DATABASE_URL
ENV DATABASE_URL=$DATABASE_URL

COPY . .

# Ensure compatibility for ARM64
RUN rustup target add aarch64-unknown-linux-gnu && \
    cargo build --release --target=aarch64-unknown-linux-gnu

# Production stage
FROM debian:bookworm-slim

WORKDIR /usr/local/bin

# Copy the compiled binary from the build stage
COPY --from=builder /app/target/aarch64-unknown-linux-gnu/release/rust-crud-api .

# Set runtime environment variable
ENV DATABASE_URL=$DATABASE_URL

# Run the application
CMD ["./rust-crud-api"]
