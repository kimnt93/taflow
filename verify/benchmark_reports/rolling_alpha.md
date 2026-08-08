# RollingAlpha benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.040 | 24.89M | 0.041 | 24.12M | nan | — | — |
| 10,000 | 0.383 | 26.14M | 0.391 | 25.56M | nan | — | — |
| 100,000 | 3.802 | 26.30M | 3.824 | 26.15M | nan | — | — |
| 1,000,000 | 38.407 | 26.04M | 36.898 | 27.10M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **3.815 ms**; native kernel **3.751 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.276 | 0.214 | 4.67M | nan | — | — |
| 100,000 | 10 | 1.908 | 1.198 | 8.35M | nan | — | — |
| 100,000 | 1,000 | 40.358 | 38.717 | 25.83M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 23.50M | 24.34M | 1.00× | 2.20M | 2.29M | 1.00× | — |
| 2 | 45.42M | 47.38M | 1.95× | 2.67M | 2.58M | 1.13× | — |
| 4 | 78.63M | 91.48M | 3.76× | 2.55M | 2.51M | 1.10× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
