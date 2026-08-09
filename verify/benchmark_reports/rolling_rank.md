# RollingRank benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 67.74M | 0.014 | 70.42M | nan | — | — |
| 10,000 | 0.129 | 77.55M | 0.128 | 78.14M | nan | — | — |
| 100,000 | 1.262 | 79.22M | 1.292 | 77.43M | nan | — | — |
| 1,000,000 | 13.524 | 73.94M | 12.323 | 81.15M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.258 ms**; native kernel **1.234 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.246 | 0.164 | 6.09M | nan | — | — |
| 100,000 | 10 | 0.956 | 0.597 | 16.74M | nan | — | — |
| 100,000 | 1,000 | 14.546 | 13.898 | 71.95M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 66.58M | 66.66M | 1.00× | 2.65M | 3.19M | 1.00× | — |
| 2 | 75.77M | 82.41M | 1.24× | 3.34M | 3.33M | 1.04× | — |
| 4 | 129.05M | 165.25M | 2.48× | 3.02M | 3.30M | 1.04× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
