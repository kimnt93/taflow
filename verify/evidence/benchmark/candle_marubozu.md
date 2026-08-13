# CandleMarubozu benchmark (`CDLMARUBOZU` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.088 | 11.41M | 0.089 | 11.23M | 0.033 | 0.38× | 0.37× |
| 10,000 | 0.692 | 14.46M | 0.751 | 13.32M | 0.127 | 0.18× | 0.17× |
| 100,000 | 6.880 | 14.53M | 6.803 | 14.70M | 1.006 | 0.15× | 0.15× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.158 | 0.152 | 0.96× |
| 1 | 5 | 0.471 | 0.452 | 0.96× |
| 1 | 10 | 0.633 | 0.920 | 1.45× |
| 10 | 1 | 0.068 | 0.086 | 1.26× |
| 10 | 5 | 0.312 | 0.430 | 1.38× |
| 10 | 10 | 0.652 | 0.898 | 1.38× |
| 100 | 1 | 0.083 | 0.087 | 1.05× |
| 100 | 5 | 0.317 | 0.424 | 1.34× |
| 100 | 10 | 0.674 | 0.944 | 1.40× |
| 1,000 | 1 | 0.147 | 0.098 | 0.66× |
| 1,000 | 5 | 0.309 | 0.491 | 1.59× |
| 1,000 | 10 | 0.696 | 1.024 | 1.47× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
