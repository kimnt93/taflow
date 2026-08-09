# HeikinAshi benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 67.31M | 0.013 | 79.71M | nan | — | — |
| 10,000 | 0.117 | 85.47M | 0.107 | 93.38M | nan | — | — |
| 100,000 | 1.166 | 85.73M | 1.024 | 97.65M | nan | — | — |
| 1,000,000 | 31.023 | 32.23M | 20.773 | 48.14M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.148 ms**; native kernel **1.026 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.439 | 0.278 | 3.60M | nan | — | — |
| 100,000 | 10 | 1.583 | 1.000 | 10.00M | nan | — | — |
| 100,000 | 1,000 | 12.453 | 11.341 | 88.17M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 64.76M | 77.61M | 1.00× | 1.72M | 2.26M | 1.00× | — |
| 2 | 62.13M | 72.23M | 0.93× | 1.73M | 2.14M | 0.95× | — |
| 4 | 60.29M | 67.35M | 0.87× | 1.64M | 2.09M | 0.92× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
