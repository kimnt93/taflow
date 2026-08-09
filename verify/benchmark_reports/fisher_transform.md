# FisherTransform benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.033 | 30.04M | 0.032 | 31.23M | nan | — | — |
| 10,000 | 0.347 | 28.78M | 0.345 | 28.97M | nan | — | — |
| 100,000 | 3.438 | 29.08M | 3.510 | 28.49M | nan | — | — |
| 1,000,000 | 35.015 | 28.56M | 34.494 | 28.99M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **3.472 ms**; native kernel **3.432 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.271 | 0.240 | 4.16M | nan | — | — |
| 100,000 | 10 | 1.702 | 1.093 | 9.15M | nan | — | — |
| 100,000 | 1,000 | 41.959 | 37.937 | 26.36M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 25.66M | 26.01M | 1.00× | 2.57M | 3.02M | 1.00× | — |
| 2 | 50.46M | 49.24M | 1.89× | 2.37M | 3.04M | 1.00× | — |
| 4 | 92.79M | 98.63M | 3.79× | 2.53M | 2.57M | 0.85× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
