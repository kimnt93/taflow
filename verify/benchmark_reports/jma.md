# JurikMovingAverage benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.088 | 11.39M | 0.088 | 11.33M | nan | — | — |
| 10,000 | 0.855 | 11.70M | 0.885 | 11.30M | nan | — | — |
| 100,000 | 9.099 | 10.99M | 8.836 | 11.32M | nan | — | — |
| 1,000,000 | 87.328 | 11.45M | 88.194 | 11.34M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **8.598 ms**; native kernel **8.368 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.314 | 0.242 | 4.14M | nan | — | — |
| 100,000 | 10 | 1.554 | 1.288 | 7.76M | nan | — | — |
| 100,000 | 1,000 | 88.570 | 82.724 | 12.09M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 11.28M | 11.55M | 1.00× | 1.86M | 2.68M | 1.00× | — |
| 2 | 10.93M | 10.55M | 0.91× | 2.14M | 2.21M | 0.83× | — |
| 4 | 10.81M | 9.30M | 0.81× | 2.28M | 2.42M | 0.91× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
