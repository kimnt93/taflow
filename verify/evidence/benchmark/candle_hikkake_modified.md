# CandleHikkakeModified benchmark (`CDLHIKKAKEMOD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 131.05M | 0.004 | 231.97M | 0.034 | 4.50× | 7.97× |
| 10,000 | 0.060 | 167.41M | 0.057 | 174.75M | 0.084 | 1.40× | 1.47× |
| 100,000 | 0.617 | 162.00M | 0.572 | 174.69M | 0.577 | 0.93× | 1.01× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.120 | 0.119 | 0.99× |
| 1 | 5 | 0.322 | 0.461 | 1.43× |
| 1 | 10 | 0.408 | 0.920 | 2.25× |
| 10 | 1 | 0.049 | 0.114 | 2.34× |
| 10 | 5 | 0.210 | 0.429 | 2.05× |
| 10 | 10 | 0.398 | 0.949 | 2.38× |
| 100 | 1 | 0.046 | 0.091 | 1.96× |
| 100 | 5 | 0.189 | 0.503 | 2.66× |
| 100 | 10 | 0.425 | 0.959 | 2.26× |
| 1,000 | 1 | 0.051 | 0.092 | 1.81× |
| 1,000 | 5 | 0.227 | 0.464 | 2.05× |
| 1,000 | 10 | 0.450 | 1.053 | 2.34× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
