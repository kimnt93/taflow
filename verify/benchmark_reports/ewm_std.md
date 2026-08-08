# ExponentiallyWeightedStandardDeviation benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 144.10M | 0.006 | 172.17M | nan | — | — |
| 10,000 | 0.052 | 193.83M | 0.045 | 224.31M | nan | — | — |
| 100,000 | 0.452 | 221.39M | 0.405 | 246.96M | nan | — | — |
| 1,000,000 | 5.182 | 192.96M | 4.485 | 222.94M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.440 ms**; native kernel **0.404 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.204 | 0.177 | 5.65M | nan | — | — |
| 100,000 | 10 | 1.006 | 0.527 | 18.97M | nan | — | — |
| 100,000 | 1,000 | 6.369 | 7.618 | 131.27M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 149.08M | 177.16M | 1.00× | 3.34M | 3.34M | 1.00× | — |
| 2 | 142.81M | 149.93M | 0.85× | 3.84M | 3.98M | 1.19× | — |
| 4 | 277.34M | 249.65M | 1.41× | 3.66M | 3.95M | 1.19× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
