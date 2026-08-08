# MathAcosh benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 81.27M | 0.011 | 88.13M | nan | — | — |
| 10,000 | 0.104 | 95.75M | 0.104 | 96.26M | nan | — | — |
| 100,000 | 1.011 | 98.87M | 1.007 | 99.30M | nan | — | — |
| 1,000,000 | 11.787 | 84.84M | 10.716 | 93.32M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.123 ms**; native kernel **0.997 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.243 | 0.153 | 6.54M | nan | — | — |
| 100,000 | 10 | 0.961 | 0.610 | 16.39M | nan | — | — |
| 100,000 | 1,000 | 12.667 | 24.290 | 41.17M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 78.76M | 88.26M | 1.00× | 2.61M | 2.46M | 1.00× | — |
| 2 | 138.87M | 160.13M | 1.81× | 2.62M | 3.30M | 1.35× | — |
| 4 | 228.76M | 281.56M | 3.19× | 2.51M | 2.89M | 1.18× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
