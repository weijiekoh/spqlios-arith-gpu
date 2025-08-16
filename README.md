# `spqlios-arith-gpu`

## Quick start

Clone this repository and run:

```bash
cargo test test_vec_znx_add -- --nocapture
```

Sample output:

```
     Running tests/vec_znx_add.rs (target/debug/deps/vec_znx_add-acfc59d12f40b771)

running 1 test
Additions per thread: 512
GPU execution time for 131072 elements: 38ms
Time taken per element: 289.92ns
test test_vec_znx_add ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.18s
```

Likely due to memory latency, the time taken per element is far greater than
what the CPU-based implementation (using AVX extension) achieves. That said, it
would be best to perform multi-stage operations entirely in the GPU in order to
minimise data transfer costs and maximise parallelism.
