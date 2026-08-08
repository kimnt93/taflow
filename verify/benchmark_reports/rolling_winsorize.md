# RollingWinsorize benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.041 | 24.23M | 0.041 | 24.60M | nan | — | — |
| 10,000 | 0.454 | 22.01M | 0.474 | 21.11M | nan | — | — |
| 100,000 | 4.771 | 20.96M | 4.558 | 21.94M | nan | — | — |
| 1,000,000 | 46.571 | 21.47M | 45.558 | 21.95M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **4.595 ms**; native kernel **4.619 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.280 | 0.206 | 4.84M | nan | — | — |
| 100,000 | 10 | 1.582 | 1.137 | 8.79M | nan | — | — |
| 100,000 | 1,000 | 71.742 | 63.004 | 15.87M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 21.10M | 22.01M | 1.00× | 2.96M | 2.63M | 1.00× | — |
| 2 | 39.82M | 43.08M | 1.96× | 2.73M | 2.88M | 1.09× | — |
| 4 | 74.84M | 84.12M | 3.82× | 2.60M | 2.74M | 1.04× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
