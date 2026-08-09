# FairValueGap benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 65.24M | 0.012 | 82.76M | nan | — | — |
| 10,000 | 0.108 | 92.28M | 0.097 | 102.64M | nan | — | — |
| 100,000 | 1.042 | 95.93M | 0.922 | 108.40M | nan | — | — |
| 1,000,000 | 26.784 | 37.34M | 10.805 | 92.55M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.040 ms**; native kernel **0.925 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.345 | 0.305 | 3.28M | nan | — | — |
| 100,000 | 10 | 2.352 | 1.202 | 8.32M | nan | — | — |
| 100,000 | 1,000 | 14.618 | 11.204 | 89.25M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 66.03M | 81.28M | 1.00× | 2.13M | 1.88M | 1.00× | — |
| 2 | 96.88M | 148.80M | 1.83× | 2.19M | 2.30M | 1.22× | — |
| 4 | 114.46M | 231.92M | 2.85× | 2.03M | 2.12M | 1.13× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
