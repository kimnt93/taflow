# CandleHomingPigeon benchmark (`CDLHOMINGPIGEON` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.023 | 44.39M | 0.019 | 52.10M | 0.036 | 1.59× | 1.86× |
| 10,000 | 0.155 | 64.44M | 0.139 | 72.17M | 0.135 | 0.87× | 0.97× |
| 100,000 | 1.489 | 67.14M | 1.274 | 78.50M | 0.997 | 0.67× | 0.78× |
| 1,000,000 | 12.958 | 77.18M | 14.428 | 69.31M | 8.407 | 0.65× | 0.58× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.097 | 0.133 | 1.37× |
| 1 | 5 | 0.353 | 0.520 | 1.47× |
| 1 | 10 | 0.573 | 1.010 | 1.76× |
| 10 | 1 | 0.070 | 0.106 | 1.51× |
| 10 | 5 | 0.307 | 0.493 | 1.61× |
| 10 | 10 | 0.586 | 0.982 | 1.68× |
| 100 | 1 | 0.066 | 0.119 | 1.81× |
| 100 | 5 | 0.355 | 0.581 | 1.64× |
| 100 | 10 | 0.615 | 1.116 | 1.81× |
| 1,000 | 1 | 0.080 | 0.109 | 1.37× |
| 1,000 | 5 | 0.332 | 0.590 | 1.78× |
| 1,000 | 10 | 0.642 | 1.062 | 1.65× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
