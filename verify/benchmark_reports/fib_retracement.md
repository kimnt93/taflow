# FibonacciRetracement benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.041 | 24.57M | 0.032 | 31.38M | nan | — | — |
| 10,000 | 0.325 | 30.78M | 0.293 | 34.17M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.047 ms**; native kernel **0.037 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.391 | 0.282 | 3.55M | nan | — | — |
| 1,500 | 10 | 1.075 | 1.043 | 9.59M | nan | — | — |
| 1,500 | 100 | 4.527 | 3.753 | 26.65M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 8.40M | 9.85M | 1.00× | 792.68K | 965.96K | 1.00× | — |
| 2 | 8.61M | 11.92M | 1.21× | 1.20M | 1.25M | 1.29× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
