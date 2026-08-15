# CandleEveningDojiStar benchmark (`CDLEVENINGDOJISTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 133.72M | 0.004 | 257.65M | 0.039 | 5.26× | 10.13× |
| 10,000 | 0.077 | 130.11M | 0.070 | 143.75M | 0.123 | 1.60× | 1.77× |
| 100,000 | 0.929 | 107.63M | 0.850 | 117.64M | 0.892 | 0.96× | 1.05× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.116 | 0.127 | 1.09× |
| 1 | 5 | 0.206 | 0.469 | 2.28× |
| 1 | 10 | 0.399 | 1.006 | 2.52× |
| 10 | 1 | 0.047 | 0.094 | 2.00× |
| 10 | 5 | 0.172 | 0.441 | 2.56× |
| 10 | 10 | 0.394 | 0.959 | 2.43× |
| 100 | 1 | 0.044 | 0.110 | 2.52× |
| 100 | 5 | 0.211 | 0.488 | 2.32× |
| 100 | 10 | 0.368 | 0.934 | 2.54× |
| 1,000 | 1 | 0.049 | 0.103 | 2.11× |
| 1,000 | 5 | 0.192 | 0.498 | 2.60× |
| 1,000 | 10 | 0.527 | 1.054 | 2.00× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
