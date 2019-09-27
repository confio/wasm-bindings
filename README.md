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
separate issues between code issues and local setup.

This is also the start of allowing CI builds

```shell
docker build -t wasmbind:nightly .

# note that all output dirs (target, wasm) are only in the docker image
# they have cached results from the build step, will incrementally recompile any changes since then
docker run --mount type=bind,src="$(pwd)",dst=/app,readonly --mount type=volume,dst=/app/target --mount type=volume,dst=/app/contracts/hasher/target --mount type=volume,dst=/app/wasm  --rm wasmbind:nightly
docker run --mount type=bind,src="$(pwd)",dst=/app,readonly --mount type=volume,dst=/app/target --mount type=volume,dst=/app/contracts/hasher/target --mount type=volume,dst=/app/wasm --rm wasmbind:nightly make test_llvm

# interactive see docker build artifacts only
docker run --rm -it wasmbind:nightly /bin/bash

# interactive
docker run --mount type=bind,src="$(pwd)",dst=/app,readonly --mount type=volume,dst=/app/target --mount type=volume,dst=/app/contracts/hasher/target --mount type=volume,dst=/app/wasm --rm -it wasmbind:nightly /bin/bash
```

## Benchmarks

I made some very rough benches on my laptop to get relative speed and cost of different backends.
These are not meant as absolute numbers, but relative comparisons should be solid.

Time to run 100 sha256 hashes in a wasm function:

```
wasm_hash_metered(100, 16, 43)                                                                            
                        time:   [346.44 us 347.69 us 349.07 us]
wasm_hash_singlepass(100, 16, 43)                                                                            
                        time:   [312.75 us 313.70 us 314.73 us]
wasm_hash_clif(100, 16, 43)                                                                            
                        time:   [106.29 us 107.58 us 109.09 us]
wasm_hash_llvm(100, 16, 43)                                                                            
                        time:   [55.469 us 55.640 us 55.832 us]
```

First, note that metered here is singlepass with gas metering, and it is about a 25% performance hit. Not bad at all.

Second, notice that clif gives ~3x speedup on singlepass, and llvm another ~2x. And the speed running llvm compiled wasm
is around 555ns per sha256 call. For reference, I benchmarked running the native go crypto/sha256 
library, which [uses hand-optimized assembly routines](https://golang.org/src/crypto/sha256/sha256block_amd64.s)
and got around `302ns/ops` on the same machine. Showing that with the llvm backed, we are
within a factor of 2 of hand-optimized solutions, which should be acceptable for
almost any workload.

Another important point is the one-time overhead to compile wasm to native code.
This only has to be done one time on contract creation, while the above is done
on every run. But it seems the execution speed bonus has a heavy upfront cost.
It may be worth digging into see where one is better than the other.

```
wasm_setup_singlepass() time:   [3.5701 ms 3.5936 ms 3.6213 ms]                       

wasm_setup_clif()       time:   [18.735 ms 19.501 ms 19.902 ms]                             

wasm_setup_llvm()       time:   [530.72 ms 532.92 ms 536.13 ms]                            
```

We see clif having a 6x compile cost for the 3x execution speedup, which seems fair.
But llvm needs an additional 25x upfront cost for the next 2x speedup.
Making it best suited for frequently executed, computationally heavy contracts,
involving eg. signature verification.

Also note that llvm is the only one to support both gas metering (singlepass and llvm)
as well as serializing the compiled modules (clif and llvm), so is our only real choice now.
If serialization is added to singlepass, or gas metering support to clif, then those would be compelling options.

But don't take my word, try it on your computer:

`cargo bench` or `cargo bench --features llvm` (if you set this up)

`cd golang && go test -bench .`