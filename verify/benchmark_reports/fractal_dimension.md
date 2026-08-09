# FractalDimension benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.063 | 15.81M | 0.062 | 16.24M | nan | — | — |
| 10,000 | 0.600 | 16.65M | 0.587 | 17.04M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.094 ms**; native kernel **0.092 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.304 | 0.234 | 4.28M | nan | — | — |
| 1,500 | 10 | 1.598 | 1.136 | 8.80M | nan | — | — |
| 1,500 | 100 | 7.516 | 6.917 | 14.46M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 6.18M | 5.76M | 1.00× | 1.11M | 1.04M | 1.00× | — |
| 2 | 14.09M | 14.46M | 2.51× | 1.35M | 1.46M | 1.40× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
