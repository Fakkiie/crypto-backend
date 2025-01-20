FROM rust:latest as builder
WORKDIR /usr/src/crypto-backend
COPY . .
RUN apt-get update && apt-get install -y libpq-dev
RUN cargo install --path .

FROM ubuntu:22.04
RUN apt-get update && apt-get install -y ca-certificates libpq-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/crypto-backend /usr/local/bin/crypto-backend
CMD ["crypto-backend"]