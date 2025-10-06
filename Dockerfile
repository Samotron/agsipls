# Multi-stage build for minimal image size
FROM rust:1.75-slim as builder

WORKDIR /build

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml ./
COPY crates/agsi-core/Cargo.toml ./crates/agsi-core/
COPY crates/agsi/Cargo.toml ./crates/agsi/

# Copy source code
COPY crates/agsi-core/src ./crates/agsi-core/src
COPY crates/agsi/src ./crates/agsi/src
COPY examples ./examples

# Build for release
RUN cargo build --release --bin agsi

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /build/target/release/agsi /usr/local/bin/agsi

# Create non-root user
RUN useradd -m -u 1000 agsi && \
    mkdir -p /data && \
    chown -R agsi:agsi /data

USER agsi
WORKDIR /data

ENTRYPOINT ["agsi"]
CMD ["--help"]
