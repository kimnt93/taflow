# CandleRiseFallThreeMethods benchmark (`CDLRISEFALL3METHODS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.040 | 25.28M | 0.039 | 25.65M | 0.047 | 1.18× | 1.20× |
| 10,000 | 0.242 | 41.24M | 0.236 | 42.44M | 0.147 | 0.61× | 0.63× |
| 100,000 | 2.179 | 45.90M | 2.181 | 45.84M | 1.115 | 0.51× | 0.51× |
| 1,000,000 | 30.318 | 32.98M | 21.502 | 46.51M | 11.526 | 0.38× | 0.54× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.084 | 0.124 | 1.48× |
| 1 | 5 | 0.320 | 0.528 | 1.65× |
| 1 | 10 | 0.713 | 1.132 | 1.59× |
| 10 | 1 | 0.062 | 0.087 | 1.41× |
| 10 | 5 | 0.294 | 0.521 | 1.77× |
| 10 | 10 | 0.682 | 1.158 | 1.70× |
| 100 | 1 | 0.068 | 0.096 | 1.42× |
| 100 | 5 | 0.316 | 0.551 | 1.74× |
| 100 | 10 | 0.694 | 1.148 | 1.65× |
| 1,000 | 1 | 0.079 | 0.110 | 1.40× |
| 1,000 | 5 | 0.321 | 0.659 | 2.05× |
| 1,000 | 10 | 0.712 | 1.248 | 1.75× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
