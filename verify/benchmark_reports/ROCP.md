# RateOfChangePercent benchmark (`ROCP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 210.60M | 0.004 | 257.99M | 0.033 | 6.96× | 8.53× |
| 10,000 | 0.020 | 492.38M | 0.018 | 555.06M | 0.044 | 2.18× | 2.46× |
| 100,000 | 0.179 | 559.03M | 0.176 | 568.06M | 0.131 | 0.73× | 0.74× |
| 1,000,000 | 2.210 | 452.52M | 1.779 | 562.22M | 1.132 | 0.51× | 0.64× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.072 | 0.116 | 1.62× |
| 1 | 5 | 0.259 | 0.550 | 2.13× |
| 1 | 10 | 0.482 | 0.930 | 1.93× |
| 10 | 1 | 0.050 | 0.096 | 1.90× |
| 10 | 5 | 0.255 | 0.553 | 2.17× |
| 10 | 10 | 0.572 | 1.222 | 2.14× |
| 100 | 1 | 0.060 | 0.115 | 1.92× |
| 100 | 5 | 0.244 | 0.477 | 1.96× |
| 100 | 10 | 0.494 | 0.968 | 1.96× |
| 1,000 | 1 | 0.052 | 0.095 | 1.83× |
| 1,000 | 5 | 0.238 | 0.474 | 2.00× |
| 1,000 | 10 | 0.532 | 1.040 | 1.95× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.248 | 0.157 | 6.38M | 123.637 | 788.22× | 198.24× |
| 100,000 | 10 | 0.848 | 0.477 | 20.96M | 131.622 | 275.87× | 63.42× |
| 100,000 | 1,000 | 7.016 | 5.768 | 173.36M | 137.235 | 23.79× | 5.97× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 304.35M | 374.00M | 1.00× | 3.67M | 3.78M | 1.00× | 491.30M |
| 5 | 713.05M | 1.05G | 2.81× | 2.92M | 3.02M | 0.80× | 461.54M |
| 10 | 678.63M | 1.17G | 3.12× | 2.56M | 2.97M | 0.79× | 483.39M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
