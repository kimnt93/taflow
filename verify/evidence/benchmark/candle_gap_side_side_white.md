# CandleGapSideSideWhite benchmark (`CDLGAPSIDESIDEWHITE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 47.95M | 0.016 | 61.36M | 0.047 | 2.26× | 2.90× |
| 10,000 | 0.133 | 75.05M | 0.134 | 74.82M | 0.239 | 1.79× | 1.79× |
| 100,000 | 1.394 | 71.75M | 1.291 | 77.48M | 2.046 | 1.47× | 1.59× |
| 1,000,000 | 13.762 | 72.66M | 13.573 | 73.68M | 20.160 | 1.46× | 1.49× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.126 | 0.160 | 1.27× |
| 1 | 5 | 0.348 | 0.456 | 1.31× |
| 1 | 10 | 0.530 | 0.984 | 1.86× |
| 10 | 1 | 0.067 | 0.098 | 1.46× |
| 10 | 5 | 0.278 | 0.480 | 1.73× |
| 10 | 10 | 0.588 | 0.891 | 1.52× |
| 100 | 1 | 0.054 | 0.091 | 1.67× |
| 100 | 5 | 0.261 | 0.529 | 2.03× |
| 100 | 10 | 0.608 | 1.034 | 1.70× |
| 1,000 | 1 | 0.074 | 0.127 | 1.71× |
| 1,000 | 5 | 0.276 | 0.607 | 2.20× |
| 1,000 | 10 | 0.691 | 1.159 | 1.68× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
