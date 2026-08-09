# RollingSkew benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.034 | 29.03M | 0.033 | 30.36M | nan | — | — |
| 10,000 | 0.311 | 32.17M | 0.309 | 32.33M | nan | — | — |
| 100,000 | 3.127 | 31.98M | 3.129 | 31.95M | nan | — | — |
| 1,000,000 | 31.371 | 31.88M | 31.327 | 31.92M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **3.136 ms**; native kernel **3.111 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.228 | 0.179 | 5.59M | nan | — | — |
| 100,000 | 10 | 1.212 | 0.799 | 12.52M | nan | — | — |
| 100,000 | 1,000 | 32.924 | 32.482 | 30.79M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 28.80M | 29.47M | 1.00× | 3.10M | 3.25M | 1.00× | — |
| 2 | 54.82M | 56.85M | 1.93× | 2.85M | 3.01M | 0.92× | — |
| 4 | 82.41M | 73.40M | 2.49× | 2.85M | 2.91M | 0.89× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
