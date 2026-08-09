# MathFloor benchmark (`FLOOR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 198.56M | 0.004 | 247.39M | 0.029 | 5.73× | 7.14× |
| 10,000 | 0.026 | 383.58M | 0.024 | 415.13M | 0.046 | 1.77× | 1.92× |
| 100,000 | 0.235 | 424.72M | 0.217 | 461.47M | 0.166 | 0.70× | 0.76× |
| 1,000,000 | 2.738 | 365.27M | 2.425 | 412.36M | 1.745 | 0.64× | 0.72× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.115 | 0.106 | 0.92× |
| 1 | 5 | 0.378 | 0.497 | 1.32× |
| 1 | 10 | 0.517 | 1.020 | 1.97× |
| 10 | 1 | 0.066 | 0.086 | 1.29× |
| 10 | 5 | 0.263 | 0.472 | 1.79× |
| 10 | 10 | 0.505 | 0.972 | 1.92× |
| 100 | 1 | 0.052 | 0.102 | 1.98× |
| 100 | 5 | 0.243 | 0.469 | 1.93× |
| 100 | 10 | 0.507 | 0.999 | 1.97× |
| 1,000 | 1 | 0.060 | 0.113 | 1.89× |
| 1,000 | 5 | 0.259 | 0.450 | 1.74× |
| 1,000 | 10 | 0.548 | 1.033 | 1.89× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.303 | 0.184 | 5.43M | 189.639 | 1028.96× | 143.06× |
| 100,000 | 10 | 1.009 | 0.556 | 17.97M | 167.351 | 300.73× | 47.05× |
| 100,000 | 1,000 | 5.178 | 3.554 | 281.37M | 164.592 | 46.31× | 8.04× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 252.06M | 297.23M | 1.00× | 3.02M | 3.79M | 1.00× | 357.32M |
| 5 | 599.70M | 861.10M | 2.90× | 2.40M | 3.20M | 0.85× | 417.50M |
| 10 | 584.13M | 1.05G | 3.52× | 2.20M | 2.92M | 0.77× | 362.17M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
