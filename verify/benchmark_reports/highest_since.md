# HighestSince benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 182.17M | 0.004 | 229.16M | nan | — | — |
| 10,000 | 0.035 | 282.81M | 0.034 | 295.11M | nan | — | — |
| 100,000 | 0.354 | 282.13M | 0.328 | 304.68M | nan | — | — |
| 1,000,000 | 3.722 | 268.65M | 3.344 | 299.01M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.351 ms**; native kernel **0.325 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.319 | 0.262 | 3.82M | nan | — | — |
| 100,000 | 10 | 1.149 | 0.702 | 14.24M | nan | — | — |
| 100,000 | 1,000 | 5.431 | 4.586 | 218.07M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 192.98M | 260.11M | 1.00× | 2.47M | 2.52M | 1.00× | — |
| 2 | 400.77M | 417.64M | 1.61× | 2.44M | 2.58M | 1.02× | — |
| 4 | 486.18M | 757.94M | 2.91× | 2.45M | 2.21M | 0.88× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
