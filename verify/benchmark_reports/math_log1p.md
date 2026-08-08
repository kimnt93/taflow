# MathLog1p benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 97.43M | 0.009 | 109.03M | nan | — | — |
| 10,000 | 0.088 | 114.09M | 0.087 | 114.87M | nan | — | — |
| 100,000 | 0.855 | 117.01M | 0.837 | 119.48M | nan | — | — |
| 1,000,000 | 9.430 | 106.05M | 8.815 | 113.44M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.007 ms**; native kernel **0.879 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.241 | 0.162 | 6.16M | nan | — | — |
| 100,000 | 10 | 1.006 | 0.616 | 16.23M | nan | — | — |
| 100,000 | 1,000 | 12.132 | 12.309 | 81.24M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 91.86M | 102.68M | 1.00× | 3.13M | 2.56M | 1.00× | — |
| 2 | 169.26M | 191.65M | 1.87× | 2.74M | 3.40M | 1.33× | — |
| 4 | 254.13M | 347.26M | 3.38× | 2.88M | 3.10M | 1.21× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
