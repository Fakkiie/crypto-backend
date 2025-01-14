FROM rust:latest as builder
WORKDIR /usr/src/axum_wasm_postgres
COPY . .
RUN cargo install --path .

ENV RPC_NETWORK_URL=https://mainnet.helius-rpc.com/?api-key=
ENV RPC_NETWORK_KEY=9470961a-e399-456d-825f-d85539c516f7

CMD ["cargo", "run", "--release"]