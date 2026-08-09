# MathCbrt benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 53.78M | 0.018 | 56.27M | nan | — | — |
| 10,000 | 0.172 | 58.25M | 0.168 | 59.44M | nan | — | — |
| 100,000 | 1.704 | 58.68M | 1.678 | 59.61M | nan | — | — |
| 1,000,000 | 17.855 | 56.01M | 17.101 | 58.48M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.710 ms**; native kernel **1.799 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.231 | 0.175 | 5.72M | nan | — | — |
| 100,000 | 10 | 1.064 | 0.685 | 14.60M | nan | — | — |
| 100,000 | 1,000 | 19.825 | 18.752 | 53.33M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 51.88M | 54.35M | 1.00× | 2.78M | 3.50M | 1.00× | — |
| 2 | 93.08M | 96.24M | 1.77× | 2.99M | 2.99M | 0.85× | — |
| 4 | 137.56M | 186.82M | 3.44× | 2.51M | 2.82M | 0.80× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
