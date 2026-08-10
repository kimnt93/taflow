# CandleKicking benchmark (`CDLKICKING` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.031 | 32.28M | 0.023 | 43.47M | 0.046 | 1.48× | 2.00× |
| 10,000 | 0.214 | 46.75M | 0.207 | 48.32M | 0.204 | 0.95× | 0.99× |
| 100,000 | 2.224 | 44.97M | 2.145 | 46.61M | 2.129 | 0.96× | 0.99× |
| 1,000,000 | 22.398 | 44.65M | 21.365 | 46.81M | 18.555 | 0.83× | 0.87× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.078 | 0.125 | 1.60× |
| 1 | 5 | 0.342 | 0.563 | 1.64× |
| 1 | 10 | 0.655 | 1.066 | 1.63× |
| 10 | 1 | 0.061 | 0.099 | 1.62× |
| 10 | 5 | 0.323 | 0.617 | 1.91× |
| 10 | 10 | 0.617 | 1.058 | 1.72× |
| 100 | 1 | 0.061 | 0.104 | 1.72× |
| 100 | 5 | 0.368 | 0.680 | 1.85× |
| 100 | 10 | 0.661 | 1.082 | 1.64× |
| 1,000 | 1 | 0.091 | 0.133 | 1.46× |
| 1,000 | 5 | 0.380 | 0.716 | 1.88× |
| 1,000 | 10 | 0.667 | 1.217 | 1.82× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
