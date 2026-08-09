# SmoothedTrendChannel benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 54.06M | 0.017 | 59.53M | nan | — | — |
| 10,000 | 0.158 | 63.16M | 0.145 | 68.75M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.026 ms**; native kernel **0.024 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.474 | 0.299 | 3.35M | nan | — | — |
| 1,500 | 10 | 1.754 | 1.413 | 7.08M | nan | — | — |
| 1,500 | 100 | 3.989 | 3.222 | 31.04M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 8.61M | 13.86M | 1.00× | 925.07K | 1.21M | 1.00× | — |
| 2 | 14.62M | 14.25M | 1.03× | 1.04M | 1.13M | 0.93× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
