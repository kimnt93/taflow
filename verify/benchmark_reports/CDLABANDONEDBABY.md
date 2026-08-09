# CandleAbandonedBaby benchmark (`CDLABANDONEDBABY` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 91.96M | 0.009 | 113.97M | 0.038 | 3.51× | 4.34× |
| 10,000 | 0.106 | 94.15M | 0.103 | 96.75M | 0.137 | 1.29× | 1.32× |
| 100,000 | 1.043 | 95.90M | 1.069 | 93.53M | 1.088 | 1.04× | 1.02× |
| 1,000,000 | 10.620 | 94.16M | 10.569 | 94.61M | 10.389 | 0.98× | 0.98× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.112 | 0.114 | 1.02× |
| 1 | 5 | 0.336 | 0.521 | 1.55× |
| 1 | 10 | 0.506 | 0.992 | 1.96× |
| 10 | 1 | 0.053 | 0.094 | 1.78× |
| 10 | 5 | 0.250 | 0.475 | 1.90× |
| 10 | 10 | 0.519 | 0.976 | 1.88× |
| 100 | 1 | 0.057 | 0.097 | 1.71× |
| 100 | 5 | 0.259 | 0.466 | 1.80× |
| 100 | 10 | 0.517 | 1.005 | 1.95× |
| 1,000 | 1 | 0.072 | 0.113 | 1.58× |
| 1,000 | 5 | 0.288 | 0.537 | 1.86× |
| 1,000 | 10 | 0.531 | 1.091 | 2.05× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
