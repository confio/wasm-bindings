# we need nightly to build wasmer
#FROM rust:1.37
FROM rustlang/rust:nightly-buster

WORKDIR /app

# add wasm support to rust
RUN rustup target add wasm32-unknown-unknown

# enable debian buster backports
RUN echo deb http://deb.debian.org/debian buster-backports main >> /etc/apt/sources.list
RUN apt-get update

# install llvm support
RUN apt-get install -y -t buster-backports libllvm8 llvm-8 llvm-8-dev llvm-8-runtime
RUN apt-get install libz-dev
ENV LLVM_SYS_80_PREFIX=/usr/lib/llvm-8

# now we want to download and compile all deps (dev mode), for faster tests when using the image
# see https://blog.mgattozzi.dev/caching-rust-docker-builds/
COPY dummy.rs /app/src/lib.rs
COPY Cargo.lock /app/
COPY Cargo.toml /app/
# we need this so Cargo.toml is valid
COPY benches/hasher.rs /app/benches/

# make sure we precompile everything. two steps for easier rebuild if the second fails
RUN cargo test
# don't run these tests, as they fail, but build all deps
RUN cargo build --tests --features llvm

# add some build tooling
COPY Makefile /app/
RUN make tools

# we now need to remove the dummy wasm_bindings compilation artifacts (as they are newer than source)
RUN rm -rf $(find target/debug -name 'wasm_bindings-*')

# prebuild wasm (only rebuild if changed since docker image built)
COPY contracts/hasher /app/contracts/hasher/
RUN make wasm

# we want to use the cached target from the build, and the src from our actual code
# docker run --mount type=bind,src="$(pwd)",dst=/app --mount type=volume,dst=/app/target --rm -it wasmbind:nightly /bin/bash
CMD ["make", "test"]
