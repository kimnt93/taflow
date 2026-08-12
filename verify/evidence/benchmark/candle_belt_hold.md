# CandleBeltHold benchmark (`CDLBELTHOLD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 48.86M | 0.018 | 55.82M | 0.038 | 1.84× | 2.10× |
| 10,000 | 0.155 | 64.66M | 0.157 | 63.71M | 0.129 | 0.84× | 0.82× |
| 100,000 | 1.585 | 63.09M | 1.532 | 65.26M | 1.042 | 0.66× | 0.68× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.117 | 0.127 | 1.08× |
| 1 | 5 | 0.361 | 0.446 | 1.24× |
| 1 | 10 | 0.542 | 0.949 | 1.75× |
| 10 | 1 | 0.058 | 0.086 | 1.46× |
| 10 | 5 | 0.251 | 0.424 | 1.69× |
| 10 | 10 | 0.526 | 0.887 | 1.69× |
| 100 | 1 | 0.054 | 0.091 | 1.68× |
| 100 | 5 | 0.299 | 0.442 | 1.48× |
| 100 | 10 | 0.524 | 0.871 | 1.66× |
| 1,000 | 1 | 0.069 | 0.105 | 1.51× |
| 1,000 | 5 | 0.261 | 0.482 | 1.85× |
| 1,000 | 10 | 0.625 | 1.051 | 1.68× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
