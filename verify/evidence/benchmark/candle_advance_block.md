# CandleAdvanceBlock benchmark (`CDLADVANCEBLOCK` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.029 | 34.66M | 0.024 | 41.33M | 0.061 | 2.10× | 2.51× |
| 10,000 | 0.214 | 46.66M | 0.210 | 47.61M | 0.262 | 1.22× | 1.25× |
| 100,000 | 2.184 | 45.78M | 2.099 | 47.64M | 2.667 | 1.22× | 1.27× |
| 1,000,000 | 24.379 | 41.02M | 21.256 | 47.05M | 30.328 | 1.24× | 1.43× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.101 | 0.155 | 1.53× |
| 1 | 5 | 3.238 | 0.808 | 0.25× |
| 1 | 10 | 1.299 | 1.350 | 1.04× |
| 10 | 1 | 0.074 | 0.105 | 1.42× |
| 10 | 5 | 0.431 | 0.727 | 1.69× |
| 10 | 10 | 1.025 | 1.365 | 1.33× |
| 100 | 1 | 0.070 | 0.097 | 1.39× |
| 100 | 5 | 0.301 | 0.572 | 1.90× |
| 100 | 10 | 0.711 | 1.166 | 1.64× |
| 1,000 | 1 | 0.076 | 0.108 | 1.43× |
| 1,000 | 5 | 0.284 | 0.638 | 2.24× |
| 1,000 | 10 | 0.757 | 1.308 | 1.73× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
