# CandleAbandonedBaby benchmark (`CDLABANDONEDBABY` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 51.42M | 0.016 | 61.03M | 0.037 | 1.89× | 2.24× |
| 10,000 | 0.159 | 63.02M | 0.149 | 67.30M | 0.130 | 0.82× | 0.87× |
| 100,000 | 1.562 | 64.01M | 1.613 | 62.00M | 0.986 | 0.63× | 0.61× |
| 1,000,000 | 15.395 | 64.95M | 17.246 | 57.99M | 9.901 | 0.64× | 0.57× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.098 | 0.110 | 1.13× |
| 1 | 5 | 0.470 | 0.460 | 0.98× |
| 1 | 10 | 0.529 | 0.968 | 1.83× |
| 10 | 1 | 0.054 | 0.104 | 1.93× |
| 10 | 5 | 0.249 | 0.507 | 2.03× |
| 10 | 10 | 0.572 | 1.024 | 1.79× |
| 100 | 1 | 0.060 | 0.096 | 1.61× |
| 100 | 5 | 0.285 | 0.458 | 1.61× |
| 100 | 10 | 0.515 | 0.941 | 1.83× |
| 1,000 | 1 | 0.074 | 0.102 | 1.37× |
| 1,000 | 5 | 0.267 | 0.505 | 1.89× |
| 1,000 | 10 | 0.540 | 1.049 | 1.94× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
