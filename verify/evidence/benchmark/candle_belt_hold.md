# CandleBeltHold benchmark (`CDLBELTHOLD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 142.41M | 0.003 | 297.65M | 0.035 | 5.03× | 10.52× |
| 10,000 | 0.100 | 99.98M | 0.098 | 102.15M | 0.124 | 1.24× | 1.26× |
| 100,000 | 1.011 | 98.87M | 1.055 | 94.78M | 1.013 | 1.00× | 0.96× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.154 | 0.172 | 1.12× |
| 1 | 5 | 0.295 | 0.479 | 1.63× |
| 1 | 10 | 0.390 | 0.871 | 2.23× |
| 10 | 1 | 0.041 | 0.083 | 2.04× |
| 10 | 5 | 0.192 | 0.436 | 2.27× |
| 10 | 10 | 0.423 | 0.909 | 2.15× |
| 100 | 1 | 0.044 | 0.092 | 2.09× |
| 100 | 5 | 0.181 | 0.426 | 2.35× |
| 100 | 10 | 0.381 | 1.104 | 2.90× |
| 1,000 | 1 | 0.058 | 0.095 | 1.62× |
| 1,000 | 5 | 0.192 | 0.484 | 2.51× |
| 1,000 | 10 | 0.429 | 0.998 | 2.33× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
