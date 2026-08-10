# RollingOmegaRatio benchmark (`OmegaRatio` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.032 | 30.77M | 0.033 | 30.48M | 0.083 | 2.54× | 2.52× |
| 10,000 | 0.312 | 32.00M | 0.311 | 32.16M | 0.610 | 1.95× | 1.96× |
| 100,000 | 3.146 | 31.79M | 3.087 | 32.39M | 5.862 | 1.86× | 1.90× |
| 1,000,000 | 32.005 | 31.25M | 31.067 | 32.19M | 58.114 | 1.82× | 1.87× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.104 | 0.120 | 1.16× |
| 1 | 5 | 0.238 | 0.390 | 1.64× |
| 1 | 10 | 0.459 | 0.796 | 1.73× |
| 10 | 1 | 0.046 | 0.086 | 1.85× |
| 10 | 5 | 0.240 | 0.479 | 2.00× |
| 10 | 10 | 0.493 | 0.814 | 1.65× |
| 100 | 1 | 0.056 | 0.084 | 1.51× |
| 100 | 5 | 0.232 | 0.411 | 1.77× |
| 100 | 10 | 0.517 | 0.926 | 1.79× |
| 1,000 | 1 | 0.092 | 0.142 | 1.54× |
| 1,000 | 5 | 0.237 | 0.680 | 2.87× |
| 1,000 | 10 | 0.503 | 1.536 | 3.05× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
