# RollingCalmar benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 50.86M | 0.020 | 50.95M | nan | — | — |
| 10,000 | 0.184 | 54.24M | 0.180 | 55.41M | nan | — | — |
| 100,000 | 1.730 | 57.79M | 1.754 | 57.00M | nan | — | — |
| 1,000,000 | 18.253 | 54.79M | 17.681 | 56.56M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **2.104 ms**; native kernel **1.848 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.231 | 0.183 | 5.45M | nan | — | — |
| 100,000 | 10 | 1.070 | 0.669 | 14.94M | nan | — | — |
| 100,000 | 1,000 | 20.108 | 18.494 | 54.07M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 49.48M | 55.99M | 1.00× | 3.31M | 3.58M | 1.00× | — |
| 2 | 76.45M | 73.25M | 1.31× | 3.28M | 3.20M | 0.89× | — |
| 4 | 132.55M | 138.41M | 2.47× | 2.92M | 3.34M | 0.93× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
