# ChaikinMoneyFlow benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 95.93M | 0.008 | 122.68M | nan | — | — |
| 10,000 | 0.070 | 143.07M | 0.063 | 157.93M | nan | — | — |
| 100,000 | 0.638 | 156.79M | 0.610 | 163.93M | nan | — | — |
| 1,000,000 | 6.898 | 144.96M | 6.811 | 146.83M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.644 ms**; native kernel **0.634 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.389 | 0.266 | 3.75M | nan | — | — |
| 100,000 | 10 | 2.428 | 1.197 | 8.36M | nan | — | — |
| 100,000 | 1,000 | 11.054 | 14.932 | 66.97M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 123.63M | 137.00M | 1.00× | 2.25M | 2.53M | 1.00× | — |
| 2 | 218.96M | 275.85M | 2.01× | 2.42M | 2.50M | 0.99× | — |
| 4 | 185.60M | 243.62M | 1.78× | 2.33M | 2.40M | 0.95× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
