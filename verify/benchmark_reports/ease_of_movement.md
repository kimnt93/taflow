# EaseOfMovement benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 172.58M | 0.004 | 257.31M | nan | — | — |
| 10,000 | 0.035 | 286.94M | 0.028 | 351.81M | nan | — | — |
| 100,000 | 0.285 | 350.61M | 0.262 | 381.70M | nan | — | — |
| 1,000,000 | 3.658 | 273.39M | 3.168 | 315.65M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.291 ms**; native kernel **0.271 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.289 | 0.222 | 4.50M | nan | — | — |
| 100,000 | 10 | 2.026 | 0.970 | 10.31M | nan | — | — |
| 100,000 | 1,000 | 6.100 | 4.565 | 219.06M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 227.04M | 257.39M | 1.00× | 2.77M | 3.08M | 1.00× | — |
| 2 | 368.92M | 416.54M | 1.62× | 2.72M | 2.89M | 0.94× | — |
| 4 | 450.29M | 730.65M | 2.84× | 2.66M | 2.62M | 0.85× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
