# CandleMorningDojiStar benchmark (`CDLMORNINGDOJISTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.132 | 7.59M | 0.119 | 8.41M | 0.037 | 0.28× | 0.31× |
| 10,000 | 1.036 | 9.65M | 1.059 | 9.44M | 0.111 | 0.11× | 0.10× |
| 100,000 | 10.310 | 9.70M | 10.135 | 9.87M | 1.159 | 0.11× | 0.11× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.101 | 0.115 | 1.14× |
| 1 | 5 | 0.381 | 0.483 | 1.27× |
| 1 | 10 | 0.626 | 0.965 | 1.54× |
| 10 | 1 | 0.071 | 0.092 | 1.30× |
| 10 | 5 | 0.307 | 0.450 | 1.47× |
| 10 | 10 | 0.667 | 0.956 | 1.43× |
| 100 | 1 | 0.077 | 0.096 | 1.24× |
| 100 | 5 | 0.312 | 0.472 | 1.52× |
| 100 | 10 | 0.700 | 1.001 | 1.43× |
| 1,000 | 1 | 0.183 | 0.113 | 0.61× |
| 1,000 | 5 | 0.411 | 0.503 | 1.22× |
| 1,000 | 10 | 0.727 | 1.035 | 1.42× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
