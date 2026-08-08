# CumulativeMinimum benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 156.20M | 0.005 | 202.21M | nan | — | — |
| 10,000 | 0.047 | 211.15M | 0.042 | 240.42M | nan | — | — |
| 100,000 | 0.427 | 234.15M | 0.386 | 258.99M | nan | — | — |
| 1,000,000 | 4.619 | 216.49M | 4.015 | 249.08M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.421 ms**; native kernel **0.396 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.209 | 0.152 | 6.59M | nan | — | — |
| 100,000 | 10 | 1.093 | 0.642 | 15.57M | nan | — | — |
| 100,000 | 1,000 | 6.601 | 5.258 | 190.20M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 196.67M | 211.55M | 1.00× | 3.81M | 3.16M | 1.00× | — |
| 2 | 280.30M | 353.48M | 1.67× | 3.60M | 4.01M | 1.27× | — |
| 4 | 445.01M | 667.19M | 3.15× | 3.42M | 3.72M | 1.18× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
