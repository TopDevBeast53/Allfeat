# This is the first stage. Here we install all the dependencies that we need in order to build the Allfeat binary.
FROM alpine:3.18 as builder

ADD . ./workdir
WORKDIR "/workdir"

COPY ./scripts/docker_mode.sh ./start.sh
RUN chmod +x start.sh

# This installs all dependencies that we need (besides Rust).
RUN apk add --update alpine-sdk clang libressl-dev llvm protoc

# This installs Rust and updates Rust to the right version.
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rust_install.sh && chmod u+x rust_install.sh && ./rust_install.sh -y && \
    . $HOME/.cargo/env && rustup update && rustup update nightly && rustup target add wasm32-unknown-unknown --toolchain nightly && rustup show

# This builds the binary.
RUN $HOME/.cargo/bin/cargo build --locked --release

# Makes the Allfeat binary accessible from anywhere.
RUN cp ./target/release/allfeat /usr/local/bin

ENV MODE="testnet"

EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]
VOLUME ["/workdir"]

CMD ["./start.sh"]