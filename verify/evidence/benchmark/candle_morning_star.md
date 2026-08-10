# CandleMorningStar benchmark (`CDLMORNINGSTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 53.98M | 0.016 | 62.63M | 0.036 | 1.92× | 2.23× |
| 10,000 | 0.134 | 74.53M | 0.136 | 73.32M | 0.106 | 0.79× | 0.78× |
| 100,000 | 1.303 | 76.73M | 1.301 | 76.89M | 0.825 | 0.63× | 0.63× |
| 1,000,000 | 14.059 | 71.13M | 14.222 | 70.31M | 8.052 | 0.57× | 0.57× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.109 | 0.107 | 0.98× |
| 1 | 5 | 0.287 | 0.493 | 1.72× |
| 1 | 10 | 0.580 | 0.974 | 1.68× |
| 10 | 1 | 0.052 | 0.096 | 1.83× |
| 10 | 5 | 0.252 | 0.463 | 1.84× |
| 10 | 10 | 0.542 | 0.962 | 1.78× |
| 100 | 1 | 0.060 | 0.096 | 1.59× |
| 100 | 5 | 0.254 | 0.452 | 1.78× |
| 100 | 10 | 0.534 | 0.991 | 1.86× |
| 1,000 | 1 | 0.067 | 0.110 | 1.63× |
| 1,000 | 5 | 0.272 | 0.495 | 1.82× |
| 1,000 | 10 | 0.558 | 1.044 | 1.87× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
