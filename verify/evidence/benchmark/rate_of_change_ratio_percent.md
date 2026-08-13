# RateOfChangeRatioPercent benchmark (`ROCR100` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.037 | 27.01M | 0.030 | 33.33M | 0.031 | 0.83× | 1.03× |
| 10,000 | 0.245 | 40.88M | 0.240 | 41.72M | 0.048 | 0.20× | 0.20× |
| 100,000 | 2.356 | 42.45M | 2.275 | 43.95M | 0.123 | 0.05× | 0.05× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.098 | 0.136 | 1.39× |
| 1 | 5 | 0.380 | 0.523 | 1.38× |
| 1 | 10 | 0.593 | 0.937 | 1.58× |
| 10 | 1 | 0.064 | 0.093 | 1.45× |
| 10 | 5 | 0.293 | 0.435 | 1.49× |
| 10 | 10 | 0.605 | 0.893 | 1.48× |
| 100 | 1 | 0.068 | 0.088 | 1.30× |
| 100 | 5 | 0.299 | 0.444 | 1.49× |
| 100 | 10 | 0.615 | 0.948 | 1.54× |
| 1,000 | 1 | 0.113 | 0.179 | 1.58× |
| 1,000 | 5 | 0.324 | 0.477 | 1.47× |
| 1,000 | 10 | 0.638 | 0.968 | 1.52× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
