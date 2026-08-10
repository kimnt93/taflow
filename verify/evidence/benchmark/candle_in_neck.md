# CandleInNeck benchmark (`CDLINNECK` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 56.89M | 0.016 | 63.85M | 0.033 | 1.85× | 2.08× |
| 10,000 | 0.146 | 68.40M | 0.150 | 66.50M | 0.116 | 0.79× | 0.77× |
| 100,000 | 1.433 | 69.80M | 1.490 | 67.10M | 0.888 | 0.62× | 0.60× |
| 1,000,000 | 13.843 | 72.24M | 16.346 | 61.18M | 8.969 | 0.65× | 0.55× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.135 | 0.144 | 1.07× |
| 1 | 5 | 0.364 | 0.487 | 1.34× |
| 1 | 10 | 0.552 | 0.878 | 1.59× |
| 10 | 1 | 0.059 | 0.087 | 1.48× |
| 10 | 5 | 0.243 | 0.452 | 1.86× |
| 10 | 10 | 0.538 | 0.900 | 1.67× |
| 100 | 1 | 0.062 | 0.094 | 1.53× |
| 100 | 5 | 0.252 | 0.422 | 1.68× |
| 100 | 10 | 0.532 | 0.910 | 1.71× |
| 1,000 | 1 | 0.066 | 0.096 | 1.46× |
| 1,000 | 5 | 0.268 | 0.476 | 1.77× |
| 1,000 | 10 | 0.558 | 0.986 | 1.77× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
