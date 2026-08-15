# CandleTriStar benchmark (`CDLTRISTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 163.82M | 0.003 | 305.36M | 0.034 | 5.57× | 10.38× |
| 10,000 | 0.044 | 226.64M | 0.040 | 250.20M | 0.086 | 1.95× | 2.16× |
| 100,000 | 0.503 | 198.78M | 0.491 | 203.47M | 0.600 | 1.19× | 1.22× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.093 | 0.132 | 1.42× |
| 1 | 5 | 0.238 | 0.467 | 1.96× |
| 1 | 10 | 0.396 | 0.909 | 2.30× |
| 10 | 1 | 0.045 | 0.088 | 1.93× |
| 10 | 5 | 0.188 | 0.412 | 2.19× |
| 10 | 10 | 0.379 | 0.908 | 2.39× |
| 100 | 1 | 0.041 | 0.093 | 2.27× |
| 100 | 5 | 0.179 | 0.418 | 2.34× |
| 100 | 10 | 0.381 | 0.884 | 2.32× |
| 1,000 | 1 | 0.044 | 0.090 | 2.04× |
| 1,000 | 5 | 0.196 | 0.470 | 2.40× |
| 1,000 | 10 | 0.394 | 0.945 | 2.40× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
