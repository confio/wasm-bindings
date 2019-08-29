.PHONY: all wasm test

all: wasm test

wasm:
	mkdir -p target/wasm
	cd contracts/hasher && cargo wasm
	cp contracts/hasher/target/wasm32-unknown-unknown/release/hasher.wasm target/wasm

test:
	cargo test