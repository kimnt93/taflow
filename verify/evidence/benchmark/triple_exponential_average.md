# TripleExponentialAverage benchmark (`T3` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 129.25M | 0.007 | 136.28M | 0.042 | 5.41× | 5.71× |
| 10,000 | 0.045 | 222.03M | 0.042 | 239.51M | 0.079 | 1.76× | 1.90× |
| 100,000 | 0.374 | 267.73M | 0.346 | 289.01M | 0.439 | 1.17× | 1.27× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.063 | 0.110 | 1.74× |
| 1 | 5 | 0.339 | 0.486 | 1.43× |
| 1 | 10 | 0.474 | 1.022 | 2.16× |
| 10 | 1 | 0.053 | 0.092 | 1.75× |
| 10 | 5 | 0.229 | 0.463 | 2.02× |
| 10 | 10 | 0.473 | 0.953 | 2.02× |
| 100 | 1 | 0.057 | 0.099 | 1.73× |
| 100 | 5 | 0.234 | 0.469 | 2.00× |
| 100 | 10 | 0.474 | 0.991 | 2.09× |
| 1,000 | 1 | 0.056 | 0.098 | 1.75× |
| 1,000 | 5 | 0.246 | 0.532 | 2.16× |
| 1,000 | 10 | 0.547 | 1.017 | 1.86× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
