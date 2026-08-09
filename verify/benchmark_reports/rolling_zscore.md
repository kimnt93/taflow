# RollingZScore benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 49.86M | 0.019 | 53.62M | nan | — | — |
| 10,000 | 0.176 | 56.98M | 0.173 | 57.85M | nan | — | — |
| 100,000 | 1.732 | 57.73M | 1.746 | 57.26M | nan | — | — |
| 1,000,000 | 17.653 | 56.65M | 17.289 | 57.84M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.728 ms**; native kernel **1.705 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.217 | 0.165 | 6.07M | nan | — | — |
| 100,000 | 10 | 1.003 | 0.861 | 11.61M | nan | — | — |
| 100,000 | 1,000 | 18.920 | 18.110 | 55.22M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 51.35M | 52.78M | 1.00× | 3.34M | 3.05M | 1.00× | — |
| 2 | 94.25M | 101.36M | 1.92× | 3.23M | 3.21M | 1.05× | — |
| 4 | 143.28M | 191.28M | 3.62× | 2.93M | 3.01M | 0.99× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
