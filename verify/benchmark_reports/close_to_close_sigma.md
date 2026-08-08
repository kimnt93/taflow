# CloseToCloseSigma benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 49.51M | 0.020 | 50.84M | nan | — | — |
| 10,000 | 0.199 | 50.19M | 0.192 | 51.97M | nan | — | — |
| 100,000 | 1.833 | 54.56M | 1.782 | 56.12M | nan | — | — |
| 1,000,000 | 18.329 | 54.56M | 18.148 | 55.10M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.860 ms**; native kernel **1.776 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.221 | 0.162 | 6.18M | nan | — | — |
| 100,000 | 10 | 1.041 | 0.654 | 15.29M | nan | — | — |
| 100,000 | 1,000 | 21.909 | 20.287 | 49.29M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 48.28M | 51.51M | 1.00× | 2.62M | 2.89M | 1.00× | — |
| 2 | 97.60M | 103.56M | 2.01× | 3.01M | 3.14M | 1.09× | — |
| 4 | 78.59M | 87.15M | 1.69× | 2.97M | 3.03M | 1.05× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
