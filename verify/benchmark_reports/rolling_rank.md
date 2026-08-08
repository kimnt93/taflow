# RollingRank benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 62.76M | 0.015 | 67.41M | nan | — | — |
| 10,000 | 0.142 | 70.44M | 0.136 | 73.30M | nan | — | — |
| 100,000 | 1.397 | 71.58M | 1.356 | 73.76M | nan | — | — |
| 1,000,000 | 14.992 | 66.70M | 14.159 | 70.63M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.418 ms**; native kernel **1.381 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.246 | 0.193 | 5.18M | nan | — | — |
| 100,000 | 10 | 1.055 | 0.665 | 15.03M | nan | — | — |
| 100,000 | 1,000 | 16.381 | 15.267 | 65.50M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 61.12M | 66.75M | 1.00× | 2.77M | 3.27M | 1.00× | — |
| 2 | 84.37M | 88.19M | 1.32× | 3.34M | 3.35M | 1.02× | — |
| 4 | 192.06M | 253.46M | 3.80× | 2.93M | 3.21M | 0.98× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
