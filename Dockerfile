# Stage 1: Builder
FROM rust:1.76-slim-bullseye AS builder

WORKDIR /app

# Install openssl-dev for reqwest and sqlx-tls
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    musl-tools \
    && rm -rf /var/lib/apt/lists/*

# Copy Cargo.toml and Cargo.lock first to leverage Docker cache
COPY Cargo.toml Cargo.lock ./

# Build dummy project to cache dependencies
# This step will download and compile all dependencies listed in Cargo.toml
# If Cargo.toml or Cargo.lock don't change, this layer will be cached.
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy the rest of the source code
COPY src ./src

# Build the application
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bullseye-slim

# Install ca-certificates for HTTPS connections
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/va-novo-projeto-va-gateway--api-gateway-em-rust ./

# Expose the port the application listens on
EXPOSE 8080

# Set environment variables for the application
ENV RUST_LOG=info
ENV SERVER_PORT=8080

# Run the application
CMD ["./va-novo-projeto-va-gateway--api-gateway-em-rust"]
