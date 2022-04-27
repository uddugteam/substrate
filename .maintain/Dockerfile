FROM  rust:1.58.0-alpine3.15 as builder
LABEL description="This is the build stage for Offchain::ipfs."

WORKDIR /offchain-ipfs

# installs tools and dependecies
RUN apk update && \
    apk add --no-cache build-base gcc cmake clang clang-dev openssl-dev \
	protoc protobuf protobuf-dev git linux-headers ca-certificates

# using
RUN rustup toolchain install nightly && \
    rustup target add wasm32-unknown-unknown --toolchain nightly && \
	rustup component add rustfmt && \
    rustup default stable && \
	rustup default 1.58.0


COPY . /offchain-ipfs

# locate the protoc binary
ENV PROTOC=/usr/bin/protoc

# -L extends the lookup path to system for libstdc++
# -C codegen option to dynamically link a C runtime
ENV RUSTFLAGS='-L/usr/lib -Ctarget-feature=-crt-static'
RUN	cargo build --release

# Default target
FROM alpine:3.15
LABEL co.equilibrium.image.vendor="Equilibrium Co." \
	co.equilibrium.image.title="offchain-ipfs" \
	co.equilibrium.image.description="IPFS-powered Substrate chain"

# install tools and dependencies
RUN apk update && apk add --no-cache ca-certificates && \
	rm -vrf /var/cache/apk/*

# Create base-path directory
RUN mkdir /offchain-ipfs

# Switch from root user and give access to node base-path directory
RUN addgroup -S ipfsgroup && adduser -S ipfs -G ipfsgroup && chown -R ipfs /offchain-ipfs
USER ipfs

# add required libraries and binaries
COPY --from=builder ["/usr/lib/libstdc++.so.6", "/usr/lib/libgcc_s.so.1", "/usr/lib/" ]
COPY --from=builder ["/offchain-ipfs/target/release/substrate", \
	"/offchain-ipfs/target/release/node-template","/usr/local/bin/"]

# check if executables work in this container
RUN /usr/local/bin/substrate --version && \
		/usr/local/bin/node-template --version

# expose ports for p2p, JSON-RPC, websocket and prometheus
EXPOSE 30333 9933 9944 9615
VOLUME ["/offchain-ipfs"]

# Default to execute node template on development chain
CMD ["/usr/local/bin/node-template", "--ws-external", "--rpc-external", "--base-path=/offchain-ipfs", "--dev"]
