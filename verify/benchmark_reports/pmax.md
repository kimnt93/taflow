# ParabolicMovingAverageStop benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.023 | 43.60M | 0.023 | 43.73M | nan | — | — |
| 10,000 | 0.213 | 46.85M | 0.182 | 55.08M | nan | — | — |
| 100,000 | 1.992 | 50.21M | 1.914 | 52.24M | nan | — | — |
| 1,000,000 | 20.480 | 48.83M | 20.460 | 48.88M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.950 ms**; native kernel **1.754 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.389 | 0.289 | 3.46M | nan | — | — |
| 100,000 | 10 | 1.553 | 1.129 | 8.86M | nan | — | — |
| 100,000 | 1,000 | 34.245 | 31.572 | 31.67M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 43.93M | 44.51M | 1.00× | 1.76M | 1.98M | 1.00× | — |
| 2 | 45.74M | 46.45M | 1.04× | 1.88M | 2.45M | 1.23× | — |
| 4 | 43.14M | 48.34M | 1.09× | 1.98M | 2.25M | 1.13× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
