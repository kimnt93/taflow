# PreviousHighLow benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 74.41M | 0.011 | 92.00M | nan | — | — |
| 10,000 | 0.101 | 98.70M | 0.091 | 109.92M | nan | — | — |
| 100,000 | 0.921 | 108.56M | 0.815 | 122.71M | nan | — | — |
| 1,000,000 | 23.863 | 41.91M | 9.041 | 110.61M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.955 ms**; native kernel **0.815 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.436 | 0.365 | 2.74M | nan | — | — |
| 100,000 | 10 | 1.793 | 0.995 | 10.05M | nan | — | — |
| 100,000 | 1,000 | 12.257 | 10.832 | 92.32M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 70.20M | 84.99M | 1.00× | 1.84M | 1.65M | 1.00× | — |
| 2 | 84.71M | 101.41M | 1.19× | 1.86M | 1.88M | 1.14× | — |
| 4 | 124.45M | 182.66M | 2.15× | 1.82M | 1.87M | 1.13× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
