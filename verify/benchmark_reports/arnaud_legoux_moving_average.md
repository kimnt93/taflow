# ArnaudLegouxMovingAverage benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 66.92M | 0.013 | 75.88M | nan | — | — |
| 10,000 | 0.124 | 80.83M | 0.117 | 85.74M | nan | — | — |
| 100,000 | 1.193 | 83.85M | 1.193 | 83.79M | nan | — | — |
| 1,000,000 | 12.696 | 78.77M | 11.632 | 85.97M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.197 ms**; native kernel **1.191 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.225 | 0.216 | 4.64M | nan | — | — |
| 100,000 | 10 | 1.029 | 0.856 | 11.69M | nan | — | — |
| 100,000 | 1,000 | 17.131 | 13.883 | 72.03M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 68.24M | 74.08M | 1.00× | 2.74M | 2.99M | 1.00× | — |
| 2 | 139.88M | 138.31M | 1.87× | 3.19M | 3.21M | 1.07× | — |
| 4 | 227.20M | 255.38M | 3.45× | 3.11M | 3.38M | 1.13× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
