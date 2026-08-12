# CandleAdvanceBlock benchmark (`CDLADVANCEBLOCK` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.039 | 25.88M | 0.022 | 45.63M | 0.051 | 1.31× | 2.31× |
| 10,000 | 0.207 | 48.28M | 0.213 | 46.95M | 0.282 | 1.36× | 1.32× |
| 100,000 | 1.969 | 50.78M | 1.942 | 51.50M | 2.000 | 1.02× | 1.03× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.074 | 0.112 | 1.52× |
| 1 | 5 | 0.306 | 0.507 | 1.66× |
| 1 | 10 | 0.518 | 0.889 | 1.72× |
| 10 | 1 | 0.052 | 0.090 | 1.71× |
| 10 | 5 | 0.240 | 0.452 | 1.89× |
| 10 | 10 | 0.549 | 0.946 | 1.72× |
| 100 | 1 | 0.059 | 0.087 | 1.48× |
| 100 | 5 | 0.286 | 0.449 | 1.57× |
| 100 | 10 | 0.579 | 0.951 | 1.64× |
| 1,000 | 1 | 0.091 | 0.117 | 1.29× |
| 1,000 | 5 | 0.292 | 0.529 | 1.82× |
| 1,000 | 10 | 0.572 | 1.236 | 2.16× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
