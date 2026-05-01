FROM rust:1.95-bullseye as builder

WORKDIR /usr/src/basic-rust-backend-ksn
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    libssl-dev \
    libpq-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/basic-rust-backend-ksn/target/release/basic-rust-backend-ksn /usr/local/bin/basic-rust-backend-ksn

CMD ["basic-rust-backend-ksn"]
