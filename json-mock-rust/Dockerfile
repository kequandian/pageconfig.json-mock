# syntax=docker/dockerfile:1.4
# Advanced Dockerfile using cargo-chef for optimal caching

FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

# Configure sparse protocol
RUN mkdir -p .cargo && \
    echo '[registries.crates-io]' > .cargo/config.toml && \
    echo 'protocol = "sparse"' >> .cargo/config.toml

# Prepare recipe (dependency analysis)
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Build dependencies
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

# Build dependencies with cache mounts
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    cargo chef cook --release --recipe-path recipe.json

# Copy source and build application
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    cargo build --release


## Runtime stage
FROM debian:bookworm-slim

# Install minimal runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN groupadd -r appuser && useradd -r -g appuser appuser

# Set working directory
WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/json-mock-rust .

# Change ownership to app user
RUN chown appuser:appuser /app/json-mock-rust

# Switch to non-root user
USER appuser

# Expose port
EXPOSE 3000

# Run the application
CMD ["./json-mock-rust"]
