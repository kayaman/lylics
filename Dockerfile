# ── Build stage ──────────────────────────────────────────────
FROM rust:1.83-slim-bookworm AS builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libssl-dev musl-tools && \
    rm -rf /var/lib/apt/lists/*

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /build
COPY Cargo.toml Cargo.lock ./
# Cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs && \
    cargo build --release --target x86_64-unknown-linux-musl && \
    rm -rf src

COPY src ./src
COPY lyrics ./lyrics
RUN touch src/main.rs && \
    cargo build --release --target x86_64-unknown-linux-musl

# ── Runtime stage ────────────────────────────────────────────
FROM gcr.io/distroless/static-debian12:nonroot

LABEL org.opencontainers.image.source="https://github.com/kayaman/lylics"
LABEL org.opencontainers.image.description="Lightweight lyrics SSE microservice"

COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/lylics /lylics

USER nonroot:nonroot
EXPOSE 3000

ENV LYLICS_HOST=0.0.0.0
ENV LYLICS_PORT=3000

ENTRYPOINT ["/lylics"]
