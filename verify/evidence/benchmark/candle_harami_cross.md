# CandleHaramiCross benchmark (`CDLHARAMICROSS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 79.67M | 0.009 | 105.27M | 0.036 | 2.91× | 3.84× |
| 10,000 | 0.134 | 74.52M | 0.133 | 75.32M | 0.135 | 1.00× | 1.02× |
| 100,000 | 1.414 | 70.72M | 1.381 | 72.40M | 1.115 | 0.79× | 0.81× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.086 | 0.130 | 1.51× |
| 1 | 5 | 0.233 | 0.479 | 2.05× |
| 1 | 10 | 0.420 | 0.921 | 2.19× |
| 10 | 1 | 0.042 | 0.094 | 2.24× |
| 10 | 5 | 0.191 | 0.437 | 2.29× |
| 10 | 10 | 0.413 | 1.012 | 2.45× |
| 100 | 1 | 0.049 | 0.090 | 1.83× |
| 100 | 5 | 0.205 | 0.446 | 2.18× |
| 100 | 10 | 0.408 | 0.977 | 2.40× |
| 1,000 | 1 | 0.061 | 0.101 | 1.65× |
| 1,000 | 5 | 0.198 | 0.481 | 2.42× |
| 1,000 | 10 | 0.417 | 0.973 | 2.33× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
