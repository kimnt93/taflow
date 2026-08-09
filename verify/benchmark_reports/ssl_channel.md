# SmoothedTrendChannel benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 72.20M | 0.013 | 79.17M | nan | — | — |
| 10,000 | 0.128 | 78.02M | 0.125 | 80.14M | nan | — | — |
| 100,000 | 1.295 | 77.24M | 1.269 | 78.81M | nan | — | — |
| 1,000,000 | 13.858 | 72.16M | 13.155 | 76.02M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.295 ms**; native kernel **1.257 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.364 | 0.260 | 3.84M | nan | — | — |
| 100,000 | 10 | 1.614 | 1.029 | 9.71M | nan | — | — |
| 100,000 | 1,000 | 15.128 | 14.122 | 70.81M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 59.50M | 68.62M | 1.00× | 1.77M | 2.35M | 1.00× | — |
| 2 | 63.61M | 66.79M | 0.97× | 2.04M | 2.46M | 1.05× | — |
| 4 | 59.90M | 64.97M | 0.95× | 1.81M | 2.36M | 1.01× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
