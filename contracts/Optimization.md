# Wasm Optimization

We don't only need fast code, but to upload to a chain, we want small code. 
A 1 MB binary is small for a normal program, but huge for a blockchain transaction.
If the `*.wasm` output is still too large after setting a few compiler flags mentioned
before, then you can try some of these approaches.

## Trimming Wasm files

Here are some tools to try out to shrink space:

* `--gc-sections` compiler flag (from [wasm-gc](https://github.com/alexcrichton/wasm-gc))
* [wasm-snip](https://github.com/rustwasm/wasm-snip) can remove specified functions
* [binaryen](https://github.com/WebAssembly/binaryen) contains a `wasm-opt` command.
Compile it or [use the docker images](https://github.com/gonowa/wasm-opt)

Note: [cargo-bloat](https://github.com/RazrFalcon/cargo-bloat) is an interesting project but only for ELF

## Using wee_alloc

After running the above optimizations, some people found alloc to be a large size overhead
and [explain how to use wee_alloc to reduce size even more](http://fitzgeraldnick.com/2018/02/09/wee-alloc.html).
Take a look at the [wee_alloc repo](https://github.com/rustwasm/wee_alloc).

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