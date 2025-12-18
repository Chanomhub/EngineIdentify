# Builder stage
FROM rust:1.75-slim-bookworm as builder

WORKDIR /usr/src/app

# Copy manifests to cache dependencies
COPY Cargo.toml Cargo.lock ./

# Create dummy src/main.rs to build dependencies
# This is a common pattern to cache cargo build dependencies
RUN mkdir src
RUN echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Now copy the actual source code
COPY src ./src
COPY engines.json ./

# Touch main.rs to ensure rebuild
RUN touch src/main.rs
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies if needed (e.g. openssl, though likely not needed for this simple app yet)
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy binary and config
COPY --from=builder /usr/src/app/target/release/engine_identify /app/engine_identify
COPY --from=builder /usr/src/app/engines.json /app/engines.json

EXPOSE 3000

CMD ["./engine_identify"]
