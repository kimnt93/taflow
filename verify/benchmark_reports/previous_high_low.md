# PreviousHighLow benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 72.04M | 0.011 | 87.27M | nan | — | — |
| 10,000 | 0.105 | 95.15M | 0.095 | 105.36M | nan | — | — |
| 100,000 | 0.981 | 101.90M | 0.976 | 102.42M | nan | — | — |
| 1,000,000 | 27.588 | 36.25M | 18.210 | 54.91M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.081 ms**; native kernel **0.914 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.432 | 0.365 | 2.74M | nan | — | — |
| 100,000 | 10 | 1.977 | 1.013 | 9.87M | nan | — | — |
| 100,000 | 1,000 | 12.014 | 10.197 | 98.06M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 73.74M | 87.48M | 1.00× | 1.81M | 1.83M | 1.00× | — |
| 2 | 107.03M | 166.46M | 1.90× | 1.96M | 1.94M | 1.06× | — |
| 4 | 112.83M | 147.86M | 1.69× | 1.85M | 1.81M | 0.99× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
