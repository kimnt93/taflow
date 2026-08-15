# CandleDoji benchmark (`CDLDOJI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 174.77M | 0.002 | 425.45M | 0.032 | 5.61× | 13.65× |
| 10,000 | 0.019 | 538.26M | 0.015 | 677.29M | 0.052 | 2.80× | 3.53× |
| 100,000 | 0.160 | 625.48M | 0.139 | 720.28M | 0.254 | 1.59× | 1.83× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.073 | 0.146 | 2.01× |
| 1 | 5 | 0.202 | 0.435 | 2.15× |
| 1 | 10 | 0.452 | 0.991 | 2.19× |
| 10 | 1 | 0.046 | 0.089 | 1.92× |
| 10 | 5 | 0.228 | 0.488 | 2.14× |
| 10 | 10 | 0.434 | 1.044 | 2.40× |
| 100 | 1 | 0.046 | 0.091 | 1.96× |
| 100 | 5 | 0.200 | 0.484 | 2.41× |
| 100 | 10 | 0.485 | 0.986 | 2.03× |
| 1,000 | 1 | 0.052 | 0.097 | 1.87× |
| 1,000 | 5 | 0.257 | 0.487 | 1.89× |
| 1,000 | 10 | 0.459 | 1.052 | 2.29× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
