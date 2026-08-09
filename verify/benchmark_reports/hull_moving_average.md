# HullMovingAverage benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 58.30M | 0.016 | 61.33M | nan | — | — |
| 10,000 | 0.152 | 65.78M | 0.151 | 66.18M | nan | — | — |
| 100,000 | 1.512 | 66.14M | 1.474 | 67.85M | nan | — | — |
| 1,000,000 | 15.131 | 66.09M | 14.834 | 67.41M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.478 ms**; native kernel **1.455 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.232 | 0.162 | 6.19M | nan | — | — |
| 100,000 | 10 | 1.028 | 0.646 | 15.48M | nan | — | — |
| 100,000 | 1,000 | 17.154 | 15.766 | 63.43M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 56.35M | 59.82M | 1.00× | 2.79M | 3.51M | 1.00× | — |
| 2 | 80.30M | 85.56M | 1.43× | 3.45M | 3.62M | 1.03× | — |
| 4 | 145.74M | 146.33M | 2.45× | 3.17M | 3.37M | 0.96× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
