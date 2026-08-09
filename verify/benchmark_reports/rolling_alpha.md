# RollingAlpha benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.040 | 24.87M | 0.037 | 27.27M | nan | — | — |
| 10,000 | 0.375 | 26.68M | 0.369 | 27.09M | nan | — | — |
| 100,000 | 3.636 | 27.51M | 3.652 | 27.38M | nan | — | — |
| 1,000,000 | 37.446 | 26.71M | 36.619 | 27.31M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **3.643 ms**; native kernel **3.654 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.257 | 0.204 | 4.90M | nan | — | — |
| 100,000 | 10 | 1.916 | 1.014 | 9.86M | nan | — | — |
| 100,000 | 1,000 | 40.886 | 36.739 | 27.22M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 24.28M | 25.30M | 1.00× | 2.43M | 2.75M | 1.00× | — |
| 2 | 47.22M | 48.60M | 1.92× | 2.62M | 2.71M | 0.99× | — |
| 4 | 87.73M | 94.20M | 3.72× | 2.54M | 2.64M | 0.96× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
