# CumulativeCount benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 245.02M | 0.003 | 332.17M | nan | — | — |
| 10,000 | 0.024 | 412.08M | 0.021 | 475.46M | nan | — | — |
| 100,000 | 0.218 | 459.30M | 0.195 | 512.00M | nan | — | — |
| 1,000,000 | 2.293 | 436.03M | 1.952 | 512.35M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.217 ms**; native kernel **0.194 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.231 | 0.146 | 6.86M | nan | — | — |
| 100,000 | 10 | 0.900 | 0.498 | 20.06M | nan | — | — |
| 100,000 | 1,000 | 4.184 | 3.289 | 304.04M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 226.99M | 304.10M | 1.00× | 3.10M | 3.83M | 1.00× | — |
| 2 | 481.78M | 574.79M | 1.89× | 3.41M | 4.11M | 1.07× | — |
| 4 | 646.14M | 1.17G | 3.83× | 3.48M | 4.00M | 1.05× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
