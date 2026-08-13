# CandleEveningDojiStar benchmark (`CDLEVENINGDOJISTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.129 | 7.75M | 0.113 | 8.86M | 0.039 | 0.30× | 0.34× |
| 10,000 | 1.025 | 9.76M | 1.032 | 9.69M | 0.114 | 0.11× | 0.11× |
| 100,000 | 10.205 | 9.80M | 10.191 | 9.81M | 0.819 | 0.08× | 0.08× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.151 | 0.121 | 0.80× |
| 1 | 5 | 0.375 | 0.567 | 1.51× |
| 1 | 10 | 0.671 | 0.985 | 1.47× |
| 10 | 1 | 0.074 | 0.101 | 1.37× |
| 10 | 5 | 0.314 | 0.458 | 1.46× |
| 10 | 10 | 0.632 | 0.955 | 1.51× |
| 100 | 1 | 0.079 | 0.099 | 1.24× |
| 100 | 5 | 0.319 | 0.473 | 1.48× |
| 100 | 10 | 0.857 | 1.127 | 1.31× |
| 1,000 | 1 | 0.203 | 0.111 | 0.55× |
| 1,000 | 5 | 0.368 | 0.499 | 1.35× |
| 1,000 | 10 | 0.732 | 1.221 | 1.67× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
