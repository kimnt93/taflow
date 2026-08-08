# ChaikinVolatility benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 105.65M | 0.007 | 139.88M | nan | — | — |
| 10,000 | 0.072 | 138.54M | 0.062 | 162.46M | nan | — | — |
| 100,000 | 0.661 | 151.27M | 0.574 | 174.23M | nan | — | — |
| 1,000,000 | 6.978 | 143.30M | 5.971 | 167.47M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.666 ms**; native kernel **0.573 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.272 | 0.242 | 4.14M | nan | — | — |
| 100,000 | 10 | 1.583 | 0.826 | 12.10M | nan | — | — |
| 100,000 | 1,000 | 9.003 | 7.599 | 131.59M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 126.24M | 152.65M | 1.00× | 2.43M | 2.40M | 1.00× | — |
| 2 | 243.71M | 295.26M | 1.93× | 2.91M | 3.17M | 1.32× | — |
| 4 | 146.05M | 194.36M | 1.27× | 3.13M | 3.16M | 1.32× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
