# TimeSeriesRank benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 63.60M | 0.015 | 67.89M | nan | — | — |
| 10,000 | 0.135 | 73.88M | 0.132 | 75.84M | nan | — | — |
| 100,000 | 1.319 | 75.83M | 1.300 | 76.95M | nan | — | — |
| 1,000,000 | 13.295 | 75.22M | 13.035 | 76.72M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.322 ms**; native kernel **1.293 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.225 | 0.171 | 5.86M | nan | — | — |
| 100,000 | 10 | 0.986 | 0.621 | 16.09M | nan | — | — |
| 100,000 | 1,000 | 15.033 | 14.385 | 69.52M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 64.00M | 67.65M | 1.00× | 2.66M | 2.67M | 1.00× | — |
| 2 | 120.19M | 131.79M | 1.95× | 3.20M | 3.06M | 1.14× | — |
| 4 | 138.83M | 162.05M | 2.40× | 3.17M | 3.27M | 1.22× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
