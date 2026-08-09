# CumulativeMinimum benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 133.60M | 0.007 | 147.51M | nan | — | — |
| 10,000 | 0.047 | 211.76M | 0.046 | 216.17M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.010 ms**; native kernel **0.009 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.239 | 0.166 | 6.02M | nan | — | — |
| 1,500 | 10 | 1.022 | 0.575 | 17.40M | nan | — | — |
| 1,500 | 100 | 2.133 | 1.590 | 62.89M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.95M | 11.38M | 1.00× | 1.25M | 1.64M | 1.00× | — |
| 2 | 20.30M | 21.46M | 1.89× | 1.53M | 1.69M | 1.03× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
