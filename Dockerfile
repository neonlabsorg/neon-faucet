ARG NEON_EVM_COMMIT=latest

FROM neonlabsorg/evm_loader:${NEON_EVM_COMMIT} AS spl
FROM solanalabs/rust:1.69.0 AS builder
RUN apt update && apt install -y libudev-dev
COPY ./src /usr/src/faucet/src
COPY ./rust-web3 /usr/src/faucet/rust-web3
COPY ./erc20 /usr/src/faucet/erc20
COPY ./Cargo.toml /usr/src/faucet
WORKDIR /usr/src/faucet
ARG REVISION
ENV FAUCET_REVISION=${REVISION}
RUN cargo build --release

FROM debian:12
RUN apt update && apt install -y ca-certificates curl
RUN mkdir -p /opt/faucet
WORKDIR /opt/faucet
ADD internal/id.json /opt/faucet/
RUN mkdir -p /root/.config/solana && ln -s /opt/faucet/id.json /root/.config/solana/id.json
ADD *.sh /opt/faucet/
ADD faucet.conf /opt/faucet/
COPY --from=builder /usr/src/faucet/target/release/faucet /opt/faucet/
RUN ln -s /opt/faucet/faucet /usr/local/bin/

COPY --from=solana /usr/bin/solana \
		/usr/bin/solana-faucet \
		/usr/bin/solana-keygen \
		/usr/bin/solana-validator \
		/usr/bin/solana-genesis \

COPY --from=spl /opt/spl-token \
		/usr/local/bin/

CMD ["/opt/faucet/faucet"]
