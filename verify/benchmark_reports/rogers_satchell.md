# RogersSatchell benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.036 | 27.60M | 0.035 | 28.79M | nan | — | — |
| 10,000 | 0.301 | 33.27M | 0.314 | 31.80M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.053 ms**; native kernel **0.050 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.724 | 0.575 | 1.74M | nan | — | — |
| 1,500 | 10 | 5.508 | 2.807 | 3.56M | nan | — | — |
| 1,500 | 100 | 11.389 | 8.300 | 12.05M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 6.23M | 8.83M | 1.00× | 1.09M | 1.15M | 1.00× | — |
| 2 | 14.31M | 16.37M | 1.85× | 1.07M | 1.21M | 1.05× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
