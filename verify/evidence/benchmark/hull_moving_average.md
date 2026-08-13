# HullMovingAverage benchmark (`HMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.331 | 3.02M | 0.334 | 2.99M | 0.158 | 0.48× | 0.47× |
| 10,000 | 3.159 | 3.17M | 3.185 | 3.14M | 0.532 | 0.17× | 0.17× |
| 100,000 | 33.105 | 3.02M | 31.458 | 3.18M | 3.983 | 0.12× | 0.13× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.111 | 0.211 | 1.89× |
| 1 | 5 | 0.474 | 0.947 | 2.00× |
| 1 | 10 | 0.631 | 2.091 | 3.32× |
| 10 | 1 | 0.071 | 0.192 | 2.72× |
| 10 | 5 | 0.306 | 0.930 | 3.04× |
| 10 | 10 | 0.633 | 2.080 | 3.28× |
| 100 | 1 | 0.104 | 0.186 | 1.78× |
| 100 | 5 | 0.296 | 0.928 | 3.13× |
| 100 | 10 | 0.635 | 2.186 | 3.44× |
| 1,000 | 1 | 0.411 | 0.231 | 0.56× |
| 1,000 | 5 | 0.631 | 1.177 | 1.86× |
| 1,000 | 10 | 1.069 | 2.479 | 2.32× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
