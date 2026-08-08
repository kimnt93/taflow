# BarsSince benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 226.66M | 0.003 | 301.79M | nan | — | — |
| 10,000 | 0.033 | 304.36M | 0.027 | 373.76M | nan | — | — |
| 100,000 | 0.269 | 371.41M | 0.246 | 406.29M | nan | — | — |
| 1,000,000 | 2.976 | 336.01M | 2.467 | 405.36M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.270 ms**; native kernel **0.249 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.310 | 0.238 | 4.21M | nan | — | — |
| 100,000 | 10 | 0.672 | 0.551 | 18.16M | nan | — | — |
| 100,000 | 1,000 | 4.068 | 3.571 | 280.06M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 251.41M | 344.54M | 1.00× | 2.53M | 2.23M | 1.00× | — |
| 2 | 405.88M | 596.09M | 1.73× | 2.72M | 2.64M | 1.19× | — |
| 4 | 550.40M | 908.79M | 2.64× | 2.83M | 2.75M | 1.23× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
