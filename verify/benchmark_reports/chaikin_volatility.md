# ChaikinVolatility benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 97.18M | 0.009 | 110.73M | nan | — | — |
| 10,000 | 0.062 | 160.62M | 0.061 | 164.22M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.014 ms**; native kernel **0.012 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.281 | 0.206 | 4.85M | nan | — | — |
| 1,500 | 10 | 1.536 | 0.819 | 12.21M | nan | — | — |
| 1,500 | 100 | 2.783 | 2.007 | 49.83M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.66M | 11.21M | 1.00× | 1.14M | 1.10M | 1.00× | — |
| 2 | 17.94M | 19.45M | 1.73× | 1.35M | 1.49M | 1.36× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
