# KalmanHedgeRatio benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 60.76M | 0.015 | 65.14M | nan | — | — |
| 10,000 | 0.143 | 70.10M | 0.138 | 72.58M | nan | — | — |
| 100,000 | 1.409 | 70.95M | 1.384 | 72.24M | nan | — | — |
| 1,000,000 | 14.682 | 68.11M | 14.387 | 69.51M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.405 ms**; native kernel **1.359 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.237 | 0.182 | 5.48M | nan | — | — |
| 100,000 | 10 | 1.516 | 0.800 | 12.49M | nan | — | — |
| 100,000 | 1,000 | 19.523 | 18.517 | 54.00M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 60.34M | 63.17M | 1.00× | 2.55M | 2.94M | 1.00× | — |
| 2 | 112.41M | 125.60M | 1.99× | 3.12M | 3.09M | 1.05× | — |
| 4 | 142.91M | 160.66M | 2.54× | 3.00M | 2.76M | 0.94× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
