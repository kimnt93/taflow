# VolumeWeightedMovingAverage benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 70.01M | 0.014 | 71.87M | nan | — | — |
| 10,000 | 0.123 | 81.14M | 0.110 | 91.22M | nan | — | — |
| 100,000 | 1.104 | 90.56M | 1.059 | 94.45M | nan | — | — |
| 1,000,000 | 11.300 | 88.50M | 10.921 | 91.56M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.093 ms**; native kernel **1.130 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.278 | 0.206 | 4.86M | nan | — | — |
| 100,000 | 10 | 1.600 | 1.024 | 9.77M | nan | — | — |
| 100,000 | 1,000 | 14.293 | 12.136 | 82.40M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 79.47M | 89.69M | 1.00× | 2.59M | 2.82M | 1.00× | — |
| 2 | 139.26M | 146.10M | 1.63× | 2.82M | 3.12M | 1.11× | — |
| 4 | 179.25M | 196.95M | 2.20× | 2.86M | 2.66M | 0.94× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
