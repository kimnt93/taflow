# CumulativeSum benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 177.53M | 0.005 | 209.80M | nan | — | — |
| 10,000 | 0.031 | 324.78M | 0.027 | 377.28M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.007 ms**; native kernel **0.006 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.225 | 0.159 | 6.28M | nan | — | — |
| 1,500 | 10 | 0.919 | 0.521 | 19.18M | nan | — | — |
| 1,500 | 100 | 1.828 | 1.312 | 76.24M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 10.76M | 15.81M | 1.00× | 1.05M | 1.03M | 1.00× | — |
| 2 | 13.73M | 16.58M | 1.05× | 1.24M | 1.71M | 1.66× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
