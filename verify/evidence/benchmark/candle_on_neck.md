# CandleOnNeck benchmark (`CDLONNECK` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.108 | 9.27M | 0.093 | 10.79M | 0.032 | 0.30× | 0.35× |
| 10,000 | 0.839 | 11.92M | 1.008 | 9.92M | 0.120 | 0.14× | 0.12× |
| 100,000 | 8.247 | 12.13M | 8.276 | 12.08M | 0.901 | 0.11× | 0.11× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.104 | 0.106 | 1.02× |
| 1 | 5 | 0.473 | 0.473 | 1.00× |
| 1 | 10 | 0.645 | 0.914 | 1.42× |
| 10 | 1 | 0.069 | 0.089 | 1.29× |
| 10 | 5 | 0.339 | 0.546 | 1.61× |
| 10 | 10 | 0.641 | 0.905 | 1.41× |
| 100 | 1 | 0.078 | 0.096 | 1.24× |
| 100 | 5 | 0.321 | 0.438 | 1.36× |
| 100 | 10 | 0.691 | 0.913 | 1.32× |
| 1,000 | 1 | 0.167 | 0.105 | 0.63× |
| 1,000 | 5 | 0.349 | 0.477 | 1.37× |
| 1,000 | 10 | 0.708 | 1.018 | 1.44× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
