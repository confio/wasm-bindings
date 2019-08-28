# Benchmarks

Running `cargo bench` in `contracts/hasher` (pure rust) gives the following:

```
hash(100, 16, 43)       time:   [29.809 us 29.935 us 30.064 us]                               
Found 8 outliers among 100 measurements (8.00%)
  5 (5.00%) low mild
  3 (3.00%) high mild
```

Running `cargo bench` in `wasm_bindings` (calling wasm function) gives the following:

```
wasm_hash(100, 16, 43)  time:   [86.686 us 87.522 us 88.431 us]                                   
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe
```