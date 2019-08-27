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
[build]
target = "wasm32-unknown-unknown"
```

And you can now just call `cargo build --release`.

## Optimizations

The size of the wasm output is critical if it is supposed to go on a blockchain.
Here are some things to make it smaller.

### Compiler Optimizations

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
Then recompile with  `cargo build --release`, and check the new size.
It should be significantly smaller. If this is too slow for your development cycle, remove these optimizations until final production.

## no_std (experimental)

Pulling in std adds a huge size to the binary. You can try to strip it out by avoiding the standard lib entirely.
This is not without problems (much code doesn't run on it), but there has been a lot of progress on porting many
data types to `core`, which has `no_std` versions of much of the rust library.
You can see a [brief overview here](https://rust-embedded.github.io/book/intro/no-std.html#summary)

You will need to import `core`, `alloc` and set this to use heap allocations at all.

Try adding 

```rust
#![no_std]
```

to the head of `src/lib.rs`. And then get this to compile....

**TODO**