# Build stage
FROM rust:1.87.0 AS builder

WORKDIR /usr/src/my_rust_service
COPY . .

RUN cargo install --path .

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/UrlShortener /usr/local/bin/UrlShortener

ENV IN_DOCKER=true

EXPOSE 8080

CMD ["UrlShortener"]