# Start from the official Rust image for building
FROM rust:latest AS builder

WORKDIR /app

# Copy manifests and build dependencies first for caching
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build the application in release mode
RUN cargo build --release

# Use a minimal base image for the final image
FROM debian:bookworm-slim
WORKDIR /app

# Install OpenSSL runtime library
RUN apt-get update && apt-get install -y --no-install-recommends libssl3 && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/wam_message_gatling /usr/local/bin/wam_message_gatling

# Set environment variables (override at runtime as needed)
ENV WAM_SERVER_URL="" \
    GATLING_MSG_NB=1 \
    GATLING_MSG_SEC=1

# Set the entrypoint
ENTRYPOINT ["/usr/local/bin/wam_message_gatling"]
