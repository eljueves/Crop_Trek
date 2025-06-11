FROM rust:1.77-slim AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libfontconfig-dev \
    libfreetype6-dev \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

COPY . .

RUN cargo build --release

# Runtime stage
FROM rust:1.77-slim AS runtime

WORKDIR /app

# Install only runtime libraries needed by plotters
RUN apt-get update && apt-get install -y \
    libfontconfig1 \
    libfreetype6 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/tracking-solution .

ENTRYPOINT ["./tracking-solution"]