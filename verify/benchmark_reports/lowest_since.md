# LowestSince benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 186.45M | 0.004 | 236.14M | nan | — | — |
| 10,000 | 0.036 | 280.62M | 0.033 | 307.66M | nan | — | — |
| 100,000 | 0.349 | 286.57M | 0.308 | 324.76M | nan | — | — |
| 1,000,000 | 3.562 | 280.75M | 3.270 | 305.83M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.333 ms**; native kernel **0.318 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.332 | 0.261 | 3.84M | nan | — | — |
| 100,000 | 10 | 1.161 | 0.731 | 13.68M | nan | — | — |
| 100,000 | 1,000 | 5.545 | 4.668 | 214.24M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 198.99M | 205.73M | 1.00× | 2.06M | 2.15M | 1.00× | — |
| 2 | 296.78M | 460.77M | 2.24× | 2.39M | 2.57M | 1.20× | — |
| 4 | 324.84M | 734.25M | 3.57× | 2.37M | 2.41M | 1.12× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
