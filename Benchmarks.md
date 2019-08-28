# Benchmarks

Running `cargo bench` in `contracts/hasher` (pure rust) gives the following:

```
hash(100, 16, 43)       time:   [29.809 us 29.935 us 30.064 us]                               
Found 8 outliers among 100 measurements (8.00%)
  5 (5.00%) low mild
  3 (3.00%) high mild
```

Running `cargo bench` in `wasm_bindings` (calling wasm function) gives the following.
Notice the difference times of the backends. Metering uses the singlepass backend,
which accounts for most of the slowdown.

```
wasm_hash_metered(100, 16, 43)                                                                            
                        time:   [318.37 us 323.48 us 328.47 us]

wasm_hash_singlepass(100, 16, 43)                                                                            
                        time:   [245.69 us 247.59 us 249.73 us]

wasm_hash_clif(100, 16, 43)                                                                            
                        time:   [77.077 us 78.192 us 79.296 us]

wasm_hash_llvm(100, 16, 43)                                                                             
                        time:   [47.579 us 48.075 us 48.620 us]
```


