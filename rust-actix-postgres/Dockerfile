# Build stage
FROM rust:latest as builder

WORKDIR /app

# Create a blank project to cache dependencies
RUN cargo init
COPY Cargo.toml Cargo.lock ./
# Generate a lock file if it doesn't exist
RUN cargo generate-lockfile
RUN cargo build --release
RUN rm src/*.rs

# Copy source code and build
COPY . .
RUN touch src/main.rs
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies (OpenSSL is required by sqlx/actix)
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/rust-actix-postgres .

EXPOSE 8080

CMD ["./rust-actix-postgres"]
