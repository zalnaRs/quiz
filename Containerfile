FROM --platform=linux/arm64 rust:1.75-bookworm AS builder

WORKDIR /app

# Cache deps first
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Build real app
COPY src ./src
RUN cargo build --release

# ---------- Runtime stage ----------
FROM --platform=linux/arm64 debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/quiz /app/quiz

EXPOSE 9090

CMD ["./quiz"]

