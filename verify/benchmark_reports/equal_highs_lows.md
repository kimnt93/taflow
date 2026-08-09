# EqualHighsLows benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.044 | 22.77M | 0.041 | 24.27M | nan | — | — |
| 10,000 | 0.442 | 22.60M | 0.432 | 23.16M | nan | — | — |
| 100,000 | 4.341 | 23.04M | 4.277 | 23.38M | nan | — | — |
| 1,000,000 | 58.852 | 16.99M | 43.686 | 22.89M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **4.376 ms**; native kernel **4.282 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.366 | 0.310 | 3.23M | nan | — | — |
| 100,000 | 10 | 2.490 | 1.542 | 6.49M | nan | — | — |
| 100,000 | 1,000 | 46.185 | 44.034 | 22.71M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 21.61M | 22.79M | 1.00× | 2.03M | 1.99M | 1.00× | — |
| 2 | 39.95M | 43.42M | 1.91× | 2.05M | 1.95M | 0.98× | — |
| 4 | 68.81M | 68.77M | 3.02× | 2.01M | 2.04M | 1.03× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
