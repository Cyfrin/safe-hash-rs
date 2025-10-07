# Multi-stage build for safe-hash-rs
# Use the official Rust image as the build environment
FROM rust:1.83-slim-bookworm AS builder

# Set the working directory
WORKDIR /usr/src/app

# Install required build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the workspace files
COPY Cargo.toml Cargo.lock ./
COPY rust-toolchain.toml ./
COPY dist-workspace.toml ./
COPY rustfmt.toml ./

# Copy the source code
COPY crates/ ./crates/

# Build the application in release mode
RUN cargo build --release --bin safe-hash

# Runtime stage - use a minimal base image
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/* \
    && update-ca-certificates

# Create a non-root user for security
RUN useradd --create-home --shell /bin/bash --user-group --uid 1000 safeuser

# Copy the binary from the builder stage
COPY --from=builder /usr/src/app/target/release/safe-hash /usr/local/bin/safe-hash

# Ensure the binary is executable
RUN chmod +x /usr/local/bin/safe-hash

# Create a directory for input files
RUN mkdir -p /app/input && chown -R safeuser:safeuser /app

# Switch to non-root user
USER safeuser

# Set the working directory
WORKDIR /app

# Set the entrypoint
ENTRYPOINT ["safe-hash"]

# Default command (show help)
CMD ["--help"]