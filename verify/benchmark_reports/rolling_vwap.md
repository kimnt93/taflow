# RollingVolumeWeightedAveragePrice benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.030 | 32.93M | 0.028 | 35.93M | nan | — | — |
| 10,000 | 0.237 | 42.24M | 0.227 | 44.05M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.041 ms**; native kernel **0.038 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.374 | 0.318 | 3.15M | nan | — | — |
| 1,500 | 10 | 2.689 | 2.960 | 3.38M | nan | — | — |
| 1,500 | 100 | 5.955 | 4.180 | 23.93M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 8.85M | 12.46M | 1.00× | 1.11M | 1.34M | 1.00× | — |
| 2 | 14.14M | 14.70M | 1.18× | 1.09M | 1.25M | 0.93× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
