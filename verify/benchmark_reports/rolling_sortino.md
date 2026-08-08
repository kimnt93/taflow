# RollingSortino benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 67.63M | 0.017 | 58.28M | nan | — | — |
| 10,000 | 0.129 | 77.49M | 0.146 | 68.35M | nan | — | — |
| 100,000 | 1.342 | 74.52M | 1.286 | 77.76M | nan | — | — |
| 1,000,000 | 13.327 | 75.04M | 16.157 | 61.89M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.346 ms**; native kernel **1.282 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.235 | 0.179 | 5.60M | nan | — | — |
| 100,000 | 10 | 1.050 | 0.642 | 15.59M | nan | — | — |
| 100,000 | 1,000 | 14.655 | 13.582 | 73.63M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 71.84M | 74.94M | 1.00× | 3.18M | 3.24M | 1.00× | — |
| 2 | 125.69M | 125.06M | 1.67× | 2.99M | 3.27M | 1.01× | — |
| 4 | 147.81M | 126.22M | 1.68× | 3.12M | 3.01M | 0.93× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
