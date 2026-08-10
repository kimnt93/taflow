# CandleAbandonedBaby benchmark (`CDLABANDONEDBABY` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.022 | 46.15M | 0.022 | 46.34M | 0.039 | 1.81× | 1.82× |
| 10,000 | 0.173 | 57.69M | 0.172 | 58.19M | 0.147 | 0.85× | 0.86× |
| 100,000 | 1.731 | 57.78M | 1.706 | 58.62M | 1.210 | 0.70× | 0.71× |
| 1,000,000 | 17.557 | 56.96M | 17.121 | 58.41M | 11.213 | 0.64× | 0.65× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.108 | 0.127 | 1.18× |
| 1 | 5 | 0.317 | 0.470 | 1.48× |
| 1 | 10 | 0.507 | 0.926 | 1.82× |
| 10 | 1 | 0.063 | 0.100 | 1.58× |
| 10 | 5 | 0.278 | 0.452 | 1.62× |
| 10 | 10 | 0.514 | 0.932 | 1.81× |
| 100 | 1 | 0.054 | 0.096 | 1.76× |
| 100 | 5 | 0.269 | 0.547 | 2.03× |
| 100 | 10 | 0.645 | 1.003 | 1.55× |
| 1,000 | 1 | 0.075 | 0.107 | 1.43× |
| 1,000 | 5 | 0.259 | 0.515 | 1.99× |
| 1,000 | 10 | 0.649 | 1.153 | 1.78× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
