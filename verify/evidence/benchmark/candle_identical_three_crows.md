# CandleIdenticalThreeCrows benchmark (`CDLIDENTICAL3CROWS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 47.19M | 0.018 | 55.63M | 0.039 | 1.85× | 2.18× |
| 10,000 | 0.150 | 66.49M | 0.139 | 71.93M | 0.159 | 1.06× | 1.15× |
| 100,000 | 1.535 | 65.16M | 1.412 | 70.81M | 0.863 | 0.56× | 0.61× |
| 1,000,000 | 14.021 | 71.32M | 14.175 | 70.55M | 8.830 | 0.63× | 0.62× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.110 | 0.112 | 1.01× |
| 1 | 5 | 0.336 | 0.520 | 1.55× |
| 1 | 10 | 0.519 | 0.934 | 1.80× |
| 10 | 1 | 0.064 | 0.089 | 1.38× |
| 10 | 5 | 0.248 | 0.427 | 1.72× |
| 10 | 10 | 0.517 | 0.882 | 1.71× |
| 100 | 1 | 0.055 | 0.091 | 1.64× |
| 100 | 5 | 0.259 | 0.436 | 1.68× |
| 100 | 10 | 0.520 | 0.891 | 1.71× |
| 1,000 | 1 | 0.073 | 0.093 | 1.28× |
| 1,000 | 5 | 0.257 | 0.468 | 1.82× |
| 1,000 | 10 | 0.558 | 1.034 | 1.85× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
