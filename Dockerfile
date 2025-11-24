# Stage 1: build
FROM rust:1.91.1-slim AS builder
WORKDIR /app

# Install build dependencies (se ne servono)
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential ca-certificates pkg-config libssl-dev curl \
 && rm -rf /var/lib/apt/lists/*

# Copia i file di progetto e build in release
COPY Cargo.toml Cargo.lock ./
# copia sorgenti in modo da sfruttare la cache di cargo
COPY src ./src

RUN cargo build --release

# Stage 2: runtime
FROM debian:bookworm-slim
# crea utente non privilegiato
RUN useradd --no-create-home --shell /bin/false appuser

# Copia certificati CA (utile per richieste HTTPS)
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates \
 && update-ca-certificates \
 && rm -rf /var/lib/apt/lists/*

# Copia il binario dal builder
COPY --from=builder /app/target/release/rust-swagger-api /usr/local/bin/rust-swagger-api

# Imposta permessi e utente
RUN chown appuser:appuser /usr/local/bin/rust-swagger-api
USER appuser

EXPOSE 3000
ENV RUST_LOG=info
CMD ["/usr/local/bin/rust-swagger-api"]