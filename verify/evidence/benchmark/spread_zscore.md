# SpreadZScore benchmark (`rolling hedged-spread z-score` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.477 | 2.10M | 0.469 | 2.13M | 0.420 | 0.88× | 0.90× |
| 10,000 | 4.666 | 2.14M | 5.134 | 1.95M | 2.970 | 0.64× | 0.58× |
| 100,000 | 47.134 | 2.12M | 47.655 | 2.10M | 33.531 | 0.71× | 0.70× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.089 | 0.172 | 1.92× |
| 1 | 5 | 0.417 | 0.817 | 1.96× |
| 1 | 10 | 0.617 | 1.530 | 2.48× |
| 10 | 1 | 0.069 | 0.149 | 2.18× |
| 10 | 5 | 0.306 | 0.720 | 2.35× |
| 10 | 10 | 0.606 | 1.495 | 2.47× |
| 100 | 1 | 0.110 | 0.247 | 2.26× |
| 100 | 5 | 0.304 | 1.402 | 4.61× |
| 100 | 10 | 0.668 | 2.857 | 4.28× |
| 1,000 | 1 | 0.569 | 0.515 | 0.91× |
| 1,000 | 5 | 0.721 | 1.751 | 2.43× |
| 1,000 | 10 | 1.362 | 4.233 | 3.11× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
