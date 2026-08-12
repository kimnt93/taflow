# CandleAbandonedBaby benchmark (`CDLABANDONEDBABY` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 49.05M | 0.018 | 56.99M | 0.038 | 1.85× | 2.15× |
| 10,000 | 0.165 | 60.42M | 0.162 | 61.78M | 0.133 | 0.80× | 0.82× |
| 100,000 | 1.658 | 60.33M | 1.693 | 59.08M | 1.050 | 0.63× | 0.62× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.091 | 0.128 | 1.40× |
| 1 | 5 | 0.305 | 0.476 | 1.56× |
| 1 | 10 | 0.535 | 0.983 | 1.84× |
| 10 | 1 | 0.059 | 0.094 | 1.59× |
| 10 | 5 | 0.240 | 0.442 | 1.84× |
| 10 | 10 | 0.629 | 1.029 | 1.64× |
| 100 | 1 | 0.058 | 0.099 | 1.70× |
| 100 | 5 | 0.272 | 0.479 | 1.76× |
| 100 | 10 | 0.738 | 1.139 | 1.54× |
| 1,000 | 1 | 0.087 | 0.105 | 1.21× |
| 1,000 | 5 | 0.288 | 0.539 | 1.87× |
| 1,000 | 10 | 0.575 | 1.190 | 2.07× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
