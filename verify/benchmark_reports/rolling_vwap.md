# RollingVolumeWeightedAveragePrice benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.025 | 40.75M | 0.023 | 44.37M | nan | — | — |
| 10,000 | 0.217 | 46.05M | 0.213 | 46.94M | nan | — | — |
| 100,000 | 2.143 | 46.66M | 2.146 | 46.60M | nan | — | — |
| 1,000,000 | 21.926 | 45.61M | 21.101 | 47.39M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **2.180 ms**; native kernel **2.102 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.340 | 0.275 | 3.64M | nan | — | — |
| 100,000 | 10 | 2.490 | 1.390 | 7.19M | nan | — | — |
| 100,000 | 1,000 | 24.605 | 29.114 | 34.35M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 41.28M | 43.72M | 1.00× | 2.20M | 2.39M | 1.00× | — |
| 2 | 80.67M | 82.56M | 1.89× | 2.19M | 2.37M | 0.99× | — |
| 4 | 138.41M | 158.70M | 3.63× | 2.17M | 2.28M | 0.95× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
