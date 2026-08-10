# CandleMorningDojiStar benchmark (`CDLMORNINGDOJISTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.024 | 41.53M | 0.019 | 53.30M | 0.044 | 1.82× | 2.34× |
| 10,000 | 0.190 | 52.59M | 0.176 | 56.84M | 0.139 | 0.73× | 0.79× |
| 100,000 | 2.167 | 46.15M | 1.951 | 51.25M | 1.074 | 0.50× | 0.55× |
| 1,000,000 | 21.012 | 47.59M | 19.702 | 50.76M | 11.967 | 0.57× | 0.61× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.084 | 0.140 | 1.66× |
| 1 | 5 | 0.287 | 0.497 | 1.73× |
| 1 | 10 | 0.647 | 1.077 | 1.66× |
| 10 | 1 | 0.068 | 0.112 | 1.63× |
| 10 | 5 | 0.280 | 0.483 | 1.72× |
| 10 | 10 | 0.591 | 1.206 | 2.04× |
| 100 | 1 | 0.060 | 0.114 | 1.91× |
| 100 | 5 | 0.325 | 0.530 | 1.63× |
| 100 | 10 | 0.619 | 1.221 | 1.97× |
| 1,000 | 1 | 0.093 | 0.118 | 1.27× |
| 1,000 | 5 | 0.316 | 0.529 | 1.68× |
| 1,000 | 10 | 0.576 | 1.179 | 2.05× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
