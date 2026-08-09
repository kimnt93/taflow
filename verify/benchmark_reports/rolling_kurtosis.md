# RollingKurtosis benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 56.21M | 0.016 | 60.62M | nan | — | — |
| 10,000 | 0.158 | 63.15M | 0.159 | 63.01M | nan | — | — |
| 100,000 | 1.594 | 62.73M | 1.533 | 65.22M | nan | — | — |
| 1,000,000 | 16.430 | 60.87M | 15.397 | 64.95M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.546 ms**; native kernel **1.532 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.208 | 0.163 | 6.13M | nan | — | — |
| 100,000 | 10 | 1.044 | 0.630 | 15.88M | nan | — | — |
| 100,000 | 1,000 | 17.610 | 16.582 | 60.30M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 53.61M | 55.85M | 1.00× | 2.59M | 2.85M | 1.00× | — |
| 2 | 80.65M | 89.09M | 1.60× | 3.19M | 3.75M | 1.31× | — |
| 4 | 148.61M | 168.11M | 3.01× | 3.00M | 3.34M | 1.17× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
