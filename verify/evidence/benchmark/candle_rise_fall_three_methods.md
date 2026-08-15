# CandleRiseFallThreeMethods benchmark (`CDLRISEFALL3METHODS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 127.53M | 0.004 | 234.78M | 0.033 | 4.22× | 7.78× |
| 10,000 | 0.092 | 109.17M | 0.088 | 113.90M | 0.119 | 1.30× | 1.35× |
| 100,000 | 1.055 | 94.76M | 1.051 | 95.18M | 0.863 | 0.82× | 0.82× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.170 | 0.102 | 0.60× |
| 1 | 5 | 0.398 | 0.440 | 1.11× |
| 1 | 10 | 0.365 | 0.880 | 2.41× |
| 10 | 1 | 0.042 | 0.086 | 2.03× |
| 10 | 5 | 0.178 | 0.442 | 2.48× |
| 10 | 10 | 0.386 | 0.896 | 2.32× |
| 100 | 1 | 0.042 | 0.085 | 2.02× |
| 100 | 5 | 0.183 | 0.442 | 2.41× |
| 100 | 10 | 0.370 | 0.882 | 2.38× |
| 1,000 | 1 | 0.057 | 0.092 | 1.62× |
| 1,000 | 5 | 0.201 | 0.467 | 2.32× |
| 1,000 | 10 | 0.392 | 0.945 | 2.41× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
