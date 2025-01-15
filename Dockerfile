FROM rust:latest as builder
WORKDIR /usr/src/axum_wasm_postgres
COPY . .
RUN cargo install --path .

ENV RPC_NETWORK_URL=https://mainnet.helius-rpc.com/?api-key=
# Add API key here
ENV RPC_NETWORK_KEY=  

CMD ["cargo", "run", "--release"]