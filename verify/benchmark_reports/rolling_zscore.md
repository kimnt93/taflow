# RollingZScore benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 49.25M | 0.020 | 49.93M | nan | — | — |
| 10,000 | 0.191 | 52.40M | 0.185 | 54.03M | nan | — | — |
| 100,000 | 1.859 | 53.79M | 1.780 | 56.17M | nan | — | — |
| 1,000,000 | 19.257 | 51.93M | 18.889 | 52.94M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.803 ms**; native kernel **1.796 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.229 | 0.183 | 5.47M | nan | — | — |
| 100,000 | 10 | 1.127 | 0.893 | 11.19M | nan | — | — |
| 100,000 | 1,000 | 21.672 | 21.825 | 45.82M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 46.60M | 52.61M | 1.00× | 2.98M | 3.38M | 1.00× | — |
| 2 | 85.17M | 99.50M | 1.89× | 2.97M | 3.38M | 1.00× | — |
| 4 | 141.87M | 158.20M | 3.01× | 3.16M | 3.09M | 0.91× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
