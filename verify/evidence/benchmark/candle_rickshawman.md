# CandleRickshawman benchmark (`CDLRICKSHAWMAN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 49.60M | 0.016 | 62.91M | 0.039 | 1.94× | 2.47× |
| 10,000 | 0.127 | 78.54M | 0.124 | 80.78M | 0.150 | 1.18× | 1.21× |
| 100,000 | 1.272 | 78.61M | 1.318 | 75.89M | 1.147 | 0.90× | 0.87× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.114 | 0.159 | 1.39× |
| 1 | 5 | 0.343 | 0.625 | 1.82× |
| 1 | 10 | 0.577 | 0.949 | 1.64× |
| 10 | 1 | 0.054 | 0.088 | 1.63× |
| 10 | 5 | 0.300 | 0.561 | 1.87× |
| 10 | 10 | 0.557 | 1.016 | 1.82× |
| 100 | 1 | 0.055 | 0.097 | 1.76× |
| 100 | 5 | 0.292 | 0.463 | 1.59× |
| 100 | 10 | 0.676 | 1.048 | 1.55× |
| 1,000 | 1 | 0.095 | 0.120 | 1.26× |
| 1,000 | 5 | 0.305 | 0.527 | 1.72× |
| 1,000 | 10 | 0.665 | 1.033 | 1.55× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
