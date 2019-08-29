# Wasm Optimization

We don't only need fast code, but to upload to a chain, we want small code. 
A 1 MB binary is small for a normal program, but huge for a blockchain transaction.
If the `*.wasm` output is still too large after setting a few compiler flags mentioned
before, then you can try some of these approaches.


## Trimming Wasm files

Some more tips from [the official book](https://rustwasm.github.io/book/reference/code-size.html)

Use [twiggy](https://github.com/rustwasm/twiggy):

```shell script
cargo install twiggy
twiggy top wasm/hasher.wasm | head -20
twiggy garbage wasm/hasher.wasm
twiggy dominators wasm/hasher.wasm | less
```

`wasm-gc` seems to still be necessary

```shell script
cargo install wasm-gc
wasm-gc wasm/hasher.wasm
```

After this, it seems `"function_names" subsection` takes 12% and `dlmalloc` 
related code around 20% (for the simple sha256 call). 

Curious how to reduce function names from 3kb.
We can also look how to trim some kb using wee_alloc

But really, 28kb is looking quite good.

### wasm-opt

[binaryen](https://github.com/WebAssembly/binaryen) contains a `wasm-opt` command.
Compile it or [use the docker images](https://github.com/confio/wasm-opt)

```
cd wasm
docker run --rm -v "$(pwd)":/data wasm-opt:latest hasher.wasm -o hasher_opt.wasm -Os
ls -l
mv hasher_opt.wasm hasher.wasm
```

Not much code size change. No significant changes in gas cost.
This seems to be done in existing rust toolchain

## More tips

Here are some other potential tools to try out to shrink space:

* [wasm-snip](https://github.com/rustwasm/wasm-snip) can remove specified functions

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