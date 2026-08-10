# KaufmanAdaptiveMovingAverage benchmark (`KAMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 137.60M | 0.012 | 81.85M | 0.066 | 9.13× | 5.43× |
| 10,000 | 0.040 | 247.70M | 0.034 | 297.93M | 0.072 | 1.78× | 2.14× |
| 100,000 | 0.342 | 292.30M | 0.324 | 309.05M | 0.463 | 1.35× | 1.43× |
| 1,000,000 | 4.214 | 237.31M | 4.152 | 240.83M | 3.611 | 0.86× | 0.87× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.119 | 0.124 | 1.05× |
| 1 | 5 | 0.279 | 0.571 | 2.04× |
| 1 | 10 | 0.522 | 1.161 | 2.22× |
| 10 | 1 | 0.051 | 0.091 | 1.77× |
| 10 | 5 | 0.234 | 0.486 | 2.08× |
| 10 | 10 | 0.569 | 1.272 | 2.23× |
| 100 | 1 | 0.068 | 0.153 | 2.26× |
| 100 | 5 | 0.347 | 0.599 | 1.73× |
| 100 | 10 | 0.583 | 1.084 | 1.86× |
| 1,000 | 1 | 0.065 | 0.111 | 1.72× |
| 1,000 | 5 | 0.304 | 0.507 | 1.67× |
| 1,000 | 10 | 0.551 | 1.272 | 2.31× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
