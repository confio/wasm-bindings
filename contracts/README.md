# Building WebAssembly Smart Contracts

The subdirectories are various examples of compiling smart contracts.
Here are some tips useful for creating your own.

## Setup

This needs to be compiled as a c dynamic lib. To do so, first generate the crate via `cargo new --lib <name>`.
Then add the following to `Cargo.toml`:

```yaml
[lib]
 crate-type = ["cdylib"]
```

## Requirements

You must support the rust target `wasm32-unknown-unknown`.

Check which ones you currently have installed via `rustup target list --installed`.
If wasm32 is not on that list, install using `rustup target add wasm32-unknown-unknown`


## Building

Go into any subdirectory, called `<contract>` from now on:

To compile the code, run  `cargo build --release --target wasm32-unknown-unknown`. 
The output will be in `target/wasm32-unknown-unknown/release/<contract>.wasm`

You probably don't want to explicitly set the target every time, so you can just
add the following to `.cargo/config`:

```yaml
[alias]
wasm = "build --release --target wasm32-unknown-unknown"
```

And you can now just call `cargo wasm` to build it, and `cargo test` to run tests.

**Note** Using `build.target` seems to force tests to use that target as well, remove this or find a work-around.
[This discussion](https://internals.rust-lang.org/t/set-default-target-for-cargo-build-but-not-for-cargo-test/9777)
and [closed PR](https://github.com/rust-lang/cargo/pull/6825) seem to suggest this will never be done.

## Optimizations

The size of the wasm output is critical if it is supposed to go on a blockchain.
Here are some things to make it smaller.

### Smaller builds

If you want to request the compiler to make smaller binaries, 
you can hit a few flags (which raise compile time significantly).
Try adding this custom profile to Cargo.toml:

```yaml
[profile.release]
opt-level = "z"
debug = false
rpath = false
lto = true
debug-assertions = false
codegen-units = 16
panic = 'unwind'
incremental = false
overflow-checks = true
```

Check the size of the original output `du -sh  target/wasm32-unknown-unknown/release/<contract>.wasm `
Then recompile with  `cargo wasm`, and check the new size.
It should be significantly smaller. If this is too slow for your development cycle, remove these optimizations until final production.

There are many more complex approaches for small builds. Those interested can look in the [optimizations document](Optimization.md)
