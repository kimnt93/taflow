# RogersSatchell benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.033 | 30.65M | 0.036 | 28.17M | nan | — | — |
| 10,000 | 0.279 | 35.79M | 0.276 | 36.28M | nan | — | — |
| 100,000 | 2.753 | 36.33M | 2.708 | 36.93M | nan | — | — |
| 1,000,000 | 28.090 | 35.60M | 27.284 | 36.65M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **2.712 ms**; native kernel **2.677 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.361 | 0.279 | 3.58M | nan | — | — |
| 100,000 | 10 | 2.700 | 1.511 | 6.62M | nan | — | — |
| 100,000 | 1,000 | 31.764 | 28.414 | 35.19M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 33.42M | 34.47M | 1.00× | 1.97M | 2.28M | 1.00× | — |
| 2 | 63.50M | 63.72M | 1.85× | 1.93M | 2.29M | 1.00× | — |
| 4 | 86.97M | 104.46M | 3.03× | 2.21M | 2.32M | 1.02× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
