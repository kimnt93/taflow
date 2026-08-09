# MovingAverageConvergenceDivergenceExtended benchmark (`MACDEXT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 67.36M | 0.014 | 72.75M | 0.061 | 4.12× | 4.45× |
| 10,000 | 0.123 | 81.32M | 0.118 | 84.53M | 0.132 | 1.07× | 1.11× |
| 100,000 | 1.185 | 84.42M | 1.115 | 89.69M | 0.884 | 0.75× | 0.79× |
| 1,000,000 | 23.249 | 43.01M | 11.080 | 90.25M | 9.283 | 0.40× | 0.84× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.077 | 0.135 | 1.74× |
| 1 | 5 | 0.277 | 0.634 | 2.29× |
| 1 | 10 | 0.554 | 1.212 | 2.19× |
| 10 | 1 | 0.054 | 0.118 | 2.19× |
| 10 | 5 | 0.233 | 0.553 | 2.37× |
| 10 | 10 | 0.554 | 1.241 | 2.24× |
| 100 | 1 | 0.055 | 0.115 | 2.09× |
| 100 | 5 | 0.247 | 0.564 | 2.28× |
| 100 | 10 | 0.536 | 1.213 | 2.26× |
| 1,000 | 1 | 0.061 | 0.121 | 1.99× |
| 1,000 | 5 | 0.244 | 0.593 | 2.42× |
| 1,000 | 10 | 0.541 | 1.260 | 2.33× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
