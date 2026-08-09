# LogReturn benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 89.86M | 0.010 | 99.17M | nan | — | — |
| 10,000 | 0.091 | 109.70M | 0.088 | 113.52M | nan | — | — |
| 100,000 | 0.895 | 111.68M | 0.873 | 114.61M | nan | — | — |
| 1,000,000 | 9.348 | 106.98M | 8.806 | 113.56M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.901 ms**; native kernel **0.870 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.214 | 0.161 | 6.20M | nan | — | — |
| 100,000 | 10 | 0.992 | 0.600 | 16.65M | nan | — | — |
| 100,000 | 1,000 | 11.726 | 10.476 | 95.45M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 97.25M | 102.74M | 1.00× | 3.48M | 3.97M | 1.00× | — |
| 2 | 192.94M | 197.52M | 1.92× | 3.72M | 3.61M | 0.91× | — |
| 4 | 266.65M | 236.42M | 2.30× | 3.59M | 3.53M | 0.89× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
