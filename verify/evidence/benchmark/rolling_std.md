# RollingStandardDeviation benchmark (`STDDEV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 188.70M | 0.005 | 219.26M | 0.036 | 6.88× | 8.00× |
| 10,000 | 0.038 | 263.11M | 0.036 | 279.46M | 0.061 | 1.59× | 1.69× |
| 100,000 | 0.359 | 278.27M | 0.352 | 284.30M | 0.302 | 0.84× | 0.86× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.082 | 0.115 | 1.40× |
| 1 | 5 | 0.254 | 0.477 | 1.88× |
| 1 | 10 | 0.373 | 0.936 | 2.51× |
| 10 | 1 | 0.043 | 0.090 | 2.11× |
| 10 | 5 | 0.195 | 0.468 | 2.40× |
| 10 | 10 | 0.376 | 0.957 | 2.54× |
| 100 | 1 | 0.045 | 0.095 | 2.13× |
| 100 | 5 | 0.208 | 0.480 | 2.31× |
| 100 | 10 | 0.442 | 0.925 | 2.09× |
| 1,000 | 1 | 0.047 | 0.092 | 1.98× |
| 1,000 | 5 | 0.201 | 0.469 | 2.34× |
| 1,000 | 10 | 0.435 | 1.018 | 2.34× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
