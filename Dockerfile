ARG SOLANA_REVISION=v1.16.14
ARG NEON_EVM_COMMIT=latest

FROM solanalabs/solana:${SOLANA_REVISION} AS solana
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

FROM debian:11
RUN apt update && apt install -y ca-certificates curl
RUN mkdir -p /opt/faucet
ADD internal/id.json /opt/faucet/
RUN mkdir -p /root/.config/solana && ln -s /opt/faucet/id.json /root/.config/solana/id.json
ADD *.sh /
ADD faucet.conf /
COPY --from=builder /usr/src/faucet/target/release/faucet /opt/faucet/
RUN ln -s /opt/faucet/faucet /usr/local/bin/

COPY --from=solana /usr/bin/solana \
		/usr/bin/solana-faucet \
		/usr/bin/solana-keygen \
		/usr/bin/solana-validator \
		/usr/bin/solana-genesis \
		/usr/local/bin/
COPY --from=spl /opt/spl-token \
		/opt/neon-cli \
		/opt/create-test-accounts.sh \
		/opt/evm_loader-keypair.json \
		/spl/bin/

# COPY --from=spl /opt/contracts/ci-tokens/owner-keypair.json /opt/faucet

COPY --from=spl /opt/spl-token \
		/usr/local/bin/

CMD ["/opt/faucet/faucet"]
