# PercentAboveMovingAverage benchmark (`PercentAboveMa` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 189.62M | 0.003 | 296.20M | 11.598 | 2199.11× | 3435.19× |
| 10,000 | 0.034 | 290.45M | 0.025 | 395.63M | 115.187 | 3345.60× | 4557.14× |
| 100,000 | 0.248 | 403.12M | 0.220 | 455.50M | 1141.423 | 4601.36× | 5199.23× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.136 | 0.298 | 2.19× |
| 1 | 5 | 0.302 | 1.115 | 3.69× |
| 1 | 10 | 0.398 | 2.087 | 5.24× |
| 10 | 1 | 0.053 | 0.323 | 6.10× |
| 10 | 5 | 0.216 | 1.824 | 8.43× |
| 10 | 10 | 0.383 | 3.183 | 8.31× |
| 100 | 1 | 0.045 | 1.391 | 30.82× |
| 100 | 5 | 0.203 | 7.292 | 35.90× |
| 100 | 10 | 0.389 | 14.155 | 36.40× |
| 1,000 | 1 | 0.066 | 11.908 | 179.10× |
| 1,000 | 5 | 0.456 | 59.651 | 130.72× |
| 1,000 | 10 | 0.497 | 116.672 | 234.84× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
