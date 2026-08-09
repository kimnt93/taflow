# CumulativeMinimum benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 176.24M | 0.005 | 209.92M | nan | — | — |
| 10,000 | 0.041 | 246.28M | 0.038 | 264.42M | nan | — | — |
| 100,000 | 0.405 | 247.02M | 0.376 | 266.06M | nan | — | — |
| 1,000,000 | 4.299 | 232.59M | 3.758 | 266.07M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.406 ms**; native kernel **0.377 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.190 | 0.150 | 6.67M | nan | — | — |
| 100,000 | 10 | 0.848 | 0.491 | 20.36M | nan | — | — |
| 100,000 | 1,000 | 5.913 | 5.017 | 199.33M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 169.64M | 211.40M | 1.00× | 3.34M | 2.93M | 1.00× | — |
| 2 | 310.27M | 409.20M | 1.94× | 3.51M | 3.90M | 1.33× | — |
| 4 | 470.45M | 710.40M | 3.36× | 3.62M | 3.95M | 1.35× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
