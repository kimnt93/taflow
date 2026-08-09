# AnchoredVolumeWeightedAveragePrice benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 67.40M | 0.013 | 78.89M | nan | — | — |
| 10,000 | 0.120 | 83.12M | 0.111 | 89.99M | nan | — | — |
| 100,000 | 1.169 | 85.56M | 1.072 | 93.30M | nan | — | — |
| 1,000,000 | 27.771 | 36.01M | 11.888 | 84.12M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.174 ms**; native kernel **1.054 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.462 | 0.409 | 2.45M | nan | — | — |
| 100,000 | 10 | 1.938 | 1.282 | 7.80M | nan | — | — |
| 100,000 | 1,000 | 17.715 | 12.362 | 80.89M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 64.68M | 74.63M | 1.00× | 1.54M | 1.65M | 1.00× | — |
| 2 | 63.67M | 72.30M | 0.97× | 1.55M | 1.59M | 0.96× | — |
| 4 | 64.25M | 72.71M | 0.97× | 1.55M | 1.60M | 0.97× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
