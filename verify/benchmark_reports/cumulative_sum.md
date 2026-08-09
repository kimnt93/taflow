# CumulativeSum benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 260.73M | 0.003 | 358.89M | 0.051 | 13.29× | 18.30× |
| 10,000 | 0.014 | 700.94M | 0.012 | 861.00M | 0.084 | 5.85× | 7.19× |
| 100,000 | 0.118 | 844.88M | 0.093 | 1.08G | 0.428 | 3.61× | 4.62× |
| 1,000,000 | 1.457 | 686.55M | 0.997 | 1.00G | 3.953 | 2.71× | 3.96× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.119 | 0.227 | 1.91× |
| 1 | 5 | 0.247 | 0.713 | 2.89× |
| 1 | 10 | 0.465 | 1.351 | 2.91× |
| 10 | 1 | 0.049 | 0.148 | 3.02× |
| 10 | 5 | 0.232 | 0.571 | 2.47× |
| 10 | 10 | 0.453 | 1.212 | 2.67× |
| 100 | 1 | 0.048 | 0.153 | 3.18× |
| 100 | 5 | 0.219 | 0.579 | 2.65× |
| 100 | 10 | 0.461 | 1.196 | 2.59× |
| 1,000 | 1 | 0.051 | 0.157 | 3.09× |
| 1,000 | 5 | 0.229 | 0.558 | 2.44× |
| 1,000 | 10 | 0.459 | 1.130 | 2.46× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.227 | 0.155 | 6.44M | nan | — | — |
| 100,000 | 10 | 0.912 | 0.581 | 17.20M | nan | — | — |
| 100,000 | 1,000 | 3.312 | 2.437 | 410.27M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 320.88M | 674.54M | 1.00× | 3.50M | 4.09M | 1.00× | — |
| 5 | 563.67M | 1.68G | 2.49× | 2.82M | 3.30M | 0.81× | — |
| 10 | 565.36M | 1.41G | 2.09× | 2.82M | 3.17M | 0.77× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
