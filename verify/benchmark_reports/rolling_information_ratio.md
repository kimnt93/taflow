# RollingInformationRatio benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.034 | 29.47M | 0.032 | 31.11M | nan | — | — |
| 10,000 | 0.329 | 30.44M | 0.339 | 29.46M | nan | — | — |
| 100,000 | 3.173 | 31.52M | 3.133 | 31.92M | nan | — | — |
| 1,000,000 | 32.217 | 31.04M | 31.596 | 31.65M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **3.123 ms**; native kernel **3.133 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.306 | 0.234 | 4.27M | nan | — | — |
| 100,000 | 10 | 1.707 | 1.037 | 9.64M | nan | — | — |
| 100,000 | 1,000 | 35.292 | 35.870 | 27.88M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 27.88M | 28.91M | 1.00× | 2.49M | 2.43M | 1.00× | — |
| 2 | 56.07M | 57.06M | 1.97× | 2.39M | 2.49M | 1.02× | — |
| 4 | 86.81M | 94.93M | 3.28× | 2.47M | 2.56M | 1.05× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
