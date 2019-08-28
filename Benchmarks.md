# Benchmarks

Running `cargo bench` in `contracts/hasher` gives the following:

```
hash(100, 16, 43)       time:   [29.809 us 29.935 us 30.064 us]                               
Found 8 outliers among 100 measurements (8.00%)
  5 (5.00%) low mild
  3 (3.00%) high mild
```

Running `cargo bench` in `wasm_bindings` gives the following:

```
hash(100, 16, 43)       time:   [29.809 us 29.935 us 30.064 us]                               
Found 8 outliers among 100 measurements (8.00%)
  5 (5.00%) low mild
  3 (3.00%) high mild
```