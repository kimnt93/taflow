# CandleKicking benchmark (`CDLKICKING` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.026 | 37.87M | 0.023 | 43.87M | 0.041 | 1.55× | 1.80× |
| 10,000 | 0.200 | 50.08M | 0.204 | 48.90M | 0.189 | 0.94× | 0.92× |
| 100,000 | 2.094 | 47.75M | 1.964 | 50.92M | 1.534 | 0.73× | 0.78× |
| 1,000,000 | 20.705 | 48.30M | 20.571 | 48.61M | 15.270 | 0.74× | 0.74× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.102 | 0.141 | 1.39× |
| 1 | 5 | 0.301 | 0.478 | 1.59× |
| 1 | 10 | 0.552 | 0.906 | 1.64× |
| 10 | 1 | 0.052 | 0.086 | 1.65× |
| 10 | 5 | 0.248 | 0.466 | 1.88× |
| 10 | 10 | 0.554 | 0.933 | 1.68× |
| 100 | 1 | 0.056 | 0.094 | 1.67× |
| 100 | 5 | 0.253 | 0.443 | 1.75× |
| 100 | 10 | 0.591 | 1.140 | 1.93× |
| 1,000 | 1 | 0.075 | 0.111 | 1.48× |
| 1,000 | 5 | 0.283 | 0.522 | 1.85× |
| 1,000 | 10 | 0.607 | 1.202 | 1.98× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
