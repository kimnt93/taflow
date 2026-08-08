# Drawdown benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 169.27M | 0.006 | 181.57M | nan | — | — |
| 10,000 | 0.045 | 220.27M | 0.041 | 244.93M | nan | — | — |
| 100,000 | 0.398 | 251.26M | 0.387 | 258.56M | nan | — | — |
| 1,000,000 | 4.738 | 211.06M | 4.049 | 246.99M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.409 ms**; native kernel **0.379 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.199 | 0.141 | 7.07M | nan | — | — |
| 100,000 | 10 | 0.918 | 0.617 | 16.20M | nan | — | — |
| 100,000 | 1,000 | 5.858 | 4.732 | 211.34M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 179.67M | 212.72M | 1.00× | 2.70M | 3.08M | 1.00× | — |
| 2 | 313.62M | 444.08M | 2.09× | 3.09M | 3.81M | 1.24× | — |
| 4 | 474.93M | 759.37M | 3.57× | 3.70M | 3.92M | 1.27× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
