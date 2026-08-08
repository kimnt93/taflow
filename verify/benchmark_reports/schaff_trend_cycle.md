# SchaffTrendCycle benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.064 | 15.72M | 0.061 | 16.37M | nan | — | — |
| 10,000 | 0.689 | 14.52M | 0.691 | 14.47M | nan | — | — |
| 100,000 | 7.006 | 14.27M | 6.853 | 14.59M | nan | — | — |
| 1,000,000 | 86.228 | 11.60M | 70.821 | 14.12M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **6.898 ms**; native kernel **6.948 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.339 | 0.277 | 3.61M | nan | — | — |
| 100,000 | 10 | 1.639 | 1.238 | 8.08M | nan | — | — |
| 100,000 | 1,000 | 70.891 | 68.802 | 14.53M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 13.79M | 14.15M | 1.00× | 2.14M | 2.19M | 1.00× | — |
| 2 | 26.01M | 27.87M | 1.97× | 2.28M | 2.13M | 0.97× | — |
| 4 | 44.89M | 38.97M | 2.75× | 2.13M | 2.19M | 1.00× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
