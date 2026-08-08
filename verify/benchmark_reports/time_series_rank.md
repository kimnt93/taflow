# TimeSeriesRank benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 64.86M | 0.016 | 62.41M | nan | — | — |
| 10,000 | 0.144 | 69.51M | 0.151 | 66.37M | nan | — | — |
| 100,000 | 1.372 | 72.86M | 1.461 | 68.47M | nan | — | — |
| 1,000,000 | 14.036 | 71.24M | 14.054 | 71.16M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.358 ms**; native kernel **1.458 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.240 | 0.257 | 3.90M | nan | — | — |
| 100,000 | 10 | 2.070 | 1.334 | 7.50M | nan | — | — |
| 100,000 | 1,000 | 27.255 | 21.972 | 45.51M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 55.57M | 69.78M | 1.00× | 3.19M | 2.89M | 1.00× | — |
| 2 | 128.94M | 139.98M | 2.01× | 3.23M | 3.19M | 1.10× | — |
| 4 | 206.03M | 209.29M | 3.00× | 2.88M | 3.10M | 1.07× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
