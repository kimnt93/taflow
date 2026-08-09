# TrueStrengthIndex benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 115.50M | 0.008 | 131.60M | nan | — | — |
| 10,000 | 0.058 | 171.77M | 0.055 | 183.31M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.012 ms**; native kernel **0.011 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.234 | 0.316 | 3.16M | nan | — | — |
| 1,500 | 10 | 0.973 | 0.677 | 14.76M | nan | — | — |
| 1,500 | 100 | 2.173 | 1.569 | 63.75M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 9.58M | 12.48M | 1.00× | 1.17M | 1.23M | 1.00× | — |
| 2 | 18.29M | 19.17M | 1.54× | 1.22M | 1.35M | 1.09× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
