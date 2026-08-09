# ChaikinVolatility benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 120.35M | 0.007 | 143.59M | nan | — | — |
| 10,000 | 0.059 | 169.55M | 0.056 | 179.43M | nan | — | — |
| 100,000 | 0.568 | 176.16M | 0.541 | 184.67M | nan | — | — |
| 1,000,000 | 6.065 | 164.88M | 5.706 | 175.27M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.563 ms**; native kernel **0.544 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.253 | 0.199 | 5.03M | nan | — | — |
| 100,000 | 10 | 1.590 | 0.784 | 12.76M | nan | — | — |
| 100,000 | 1,000 | 8.211 | 7.003 | 142.79M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 131.65M | 165.45M | 1.00× | 3.04M | 3.32M | 1.00× | — |
| 2 | 243.14M | 261.61M | 1.58× | 3.19M | 3.33M | 1.00× | — |
| 4 | 367.67M | 544.54M | 3.29× | 3.11M | 3.22M | 0.97× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
