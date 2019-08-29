.PHONY: all wasm test test_llvm tools

all: wasm test

test: wasm
	cargo test

test_llvm: wasm
	cargo test --features llvm

wasm: wasm/hasher.wasm

wasm/hasher.wasm: contracts/hasher/src/lib.rs contracts/hasher/Cargo.toml
	mkdir -p wasm
	cd contracts/hasher && cargo wasm
	cp contracts/hasher/target/wasm32-unknown-unknown/release/hasher.wasm wasm
	wasm-gc wasm/hasher.wasm

tools:
	@ which wasm-gc || cargo install wasm-gc
	@ which twiggy || cargo install twiggy