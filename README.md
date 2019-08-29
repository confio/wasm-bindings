# Wasm Bindings

## Smart Contracts

Under `contracts/*`. Go to a subdir and run `cargo wasm` to build it's wasm

## Backends

Most supported is SinglePass.

Cranelift is fast, but doens't support middleware (aka metering).

LLVM is also fast, a longer compile time and a bit more setup.

This repo supports llvm under a feature flag, here is how to install it:

### Install LLVM

Either:
* Look at [Wasmer's install script](https://github.com/wasmerio/wasmer/pull/656/files#diff-c44d2f45c07a359dfab85f19448014efR23-R27)
* Use [llvmenv](https://crates.io/crates/llvmenv)
* Install llvm via `apt`

**Demo with apt-get:**

```
sudo apt-get install libllvm8 llvm-8 llvm-8-dev llvm-8-runtime
sudo apt-get install libz-dev
export LLVM_SYS_80_PREFIX=/usr/lib/llvm-8

# maybe??
# sudo apt-get install lldb-8  lld-8 libc++-8-dev libc++abi-8-dev
# export PATH=/usr/lib/llvm-8/bin:$PATH
```

Now test it out:

````
cargo build --features llvm
cargo test --features llvm
cargo bench --features llvm
````

## Docker support

If you are not running a linux system or don't have rust tooling installed, 
the simplest approach is likely to use the Dockerfile. 

This is also useful for reproduceable build when submitting bug reports.
Before submitting a bug, please `rm -rf target` and then run the tests
in docker to verify that they fail in a standard environment. This helps
separate issues between 

```shell
docker build -t wasmbind:nightly .
docker run --rm -it -v"$(pwd)":/app wasmbind:nightly make test
docker run --rm -it -v"$(pwd)":/app wasmbind:nightly make test_llvm
```