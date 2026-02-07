#FROM --platform=linux/arm64 rust:1.75-bookworm AS builder

#WORKDIR /app

## Cache deps first
#COPY Cargo.toml Cargo.lock ./
#RUN cargo build --release
#RUN rm -rf src

# Build real app
#COPY src ./src
#RUN cargo build --release

# ---------- Runtime stage ----------
#FROM --platform=linux/arm64 debian:bookworm-slim

#RUN apt-get update && \
#    apt-get install -y ca-certificates && \
#    rm -rf /var/lib/apt/lists/*

#WORKDIR /app

#COPY --from=builder /app/target/release/quiz /app/quiz

#EXPOSE 9090

#CMD ["./quiz"]

FROM --platform=linux/arm64 rust:1.75-alpine AS builder
RUN apk add --no-cache musl-dev

WORKDIR /app
COPY . .
RUN cargo build --release --target aarch64-unknown-linux-musl

FROM scratch
COPY --from=builder /app/target/aarch64-unknown-linux-musl/release/quiz /
EXPOSE 9090
CMD ["/quiz"]

