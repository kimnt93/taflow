# KalmanHedgeRatio benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 62.66M | 0.015 | 65.24M | nan | — | — |
| 10,000 | 0.141 | 71.15M | 0.140 | 71.43M | nan | — | — |
| 100,000 | 1.405 | 71.19M | 1.365 | 73.26M | nan | — | — |
| 1,000,000 | 14.213 | 70.36M | 13.895 | 71.97M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.380 ms**; native kernel **1.351 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.251 | 0.177 | 5.64M | nan | — | — |
| 100,000 | 10 | 1.427 | 0.779 | 12.84M | nan | — | — |
| 100,000 | 1,000 | 16.059 | 15.151 | 66.00M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 62.42M | 61.56M | 1.00× | 2.78M | 3.19M | 1.00× | — |
| 2 | 107.45M | 119.59M | 1.94× | 3.05M | 2.92M | 0.92× | — |
| 4 | 191.52M | 227.65M | 3.70× | 2.80M | 2.92M | 0.92× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
