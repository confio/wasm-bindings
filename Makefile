.PHONY: all wasm test test_llvm

all: wasm test

wasm:
	mkdir -p target/wasm
	cd contracts/hasher && cargo wasm
	cp contracts/hasher/target/wasm32-unknown-unknown/release/hasher.wasm target/wasm

test: wasm
	cargo test

test_llvm: wasm
	cargo test --features llvm