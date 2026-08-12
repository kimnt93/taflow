# CandleHammer benchmark (`CDLHAMMER` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 74.29M | 0.010 | 98.29M | 0.042 | 3.12× | 4.12× |
| 10,000 | 0.115 | 86.91M | 0.110 | 90.70M | 0.174 | 1.51× | 1.58× |
| 100,000 | 1.254 | 79.74M | 1.248 | 80.10M | 1.488 | 1.19× | 1.19× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.123 | 0.145 | 1.18× |
| 1 | 5 | 0.370 | 0.478 | 1.29× |
| 1 | 10 | 0.522 | 0.921 | 1.77× |
| 10 | 1 | 0.063 | 0.087 | 1.39× |
| 10 | 5 | 0.266 | 0.451 | 1.69× |
| 10 | 10 | 0.566 | 0.906 | 1.60× |
| 100 | 1 | 0.061 | 0.094 | 1.52× |
| 100 | 5 | 0.278 | 0.446 | 1.61× |
| 100 | 10 | 0.581 | 0.919 | 1.58× |
| 1,000 | 1 | 0.071 | 0.106 | 1.51× |
| 1,000 | 5 | 0.260 | 0.494 | 1.90× |
| 1,000 | 10 | 0.553 | 1.114 | 2.01× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
