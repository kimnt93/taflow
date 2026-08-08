# RollingVolumeWeightedAveragePrice benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.027 | 37.12M | 0.025 | 40.31M | nan | — | — |
| 10,000 | 0.230 | 43.55M | 0.230 | 43.43M | nan | — | — |
| 100,000 | 2.354 | 42.48M | 2.260 | 44.24M | nan | — | — |
| 1,000,000 | 22.970 | 43.53M | 22.905 | 43.66M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **2.324 ms**; native kernel **2.271 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.351 | 0.287 | 3.48M | nan | — | — |
| 100,000 | 10 | 2.699 | 1.401 | 7.14M | nan | — | — |
| 100,000 | 1,000 | 30.637 | 25.336 | 39.47M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 38.90M | 41.71M | 1.00× | 2.19M | 2.12M | 1.00× | — |
| 2 | 68.40M | 81.45M | 1.95× | 2.13M | 2.39M | 1.13× | — |
| 4 | 133.98M | 140.43M | 3.37× | 2.13M | 2.20M | 1.04× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
