# CumulativeSum benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 212.72M | 0.004 | 279.85M | nan | — | — |
| 10,000 | 0.030 | 336.94M | 0.026 | 383.92M | nan | — | — |
| 100,000 | 0.257 | 388.58M | 0.238 | 419.76M | nan | — | — |
| 1,000,000 | 2.970 | 336.75M | 2.581 | 387.38M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.255 ms**; native kernel **0.231 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.208 | 0.153 | 6.52M | nan | — | — |
| 100,000 | 10 | 0.873 | 0.502 | 19.93M | nan | — | — |
| 100,000 | 1,000 | 4.153 | 3.440 | 290.70M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 266.53M | 309.12M | 1.00× | 3.56M | 3.94M | 1.00× | — |
| 2 | 442.87M | 607.99M | 1.97× | 3.43M | 3.80M | 0.96× | — |
| 4 | 343.16M | 815.00M | 2.64× | 3.72M | 4.10M | 1.04× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
