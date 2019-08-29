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

# docker run -d  -v "$(pwd)":/app
CMD ["make", "test"]
