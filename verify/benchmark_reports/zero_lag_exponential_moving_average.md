# ZeroLagExponentialMovingAverage benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 165.93M | 0.005 | 197.76M | nan | — | — |
| 10,000 | 0.040 | 250.89M | 0.037 | 267.40M | nan | — | — |
| 100,000 | 0.398 | 251.38M | 0.364 | 274.98M | nan | — | — |
| 1,000,000 | 4.089 | 244.54M | 3.713 | 269.31M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.391 ms**; native kernel **0.364 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.205 | 0.140 | 7.14M | nan | — | — |
| 100,000 | 10 | 0.894 | 0.516 | 19.37M | nan | — | — |
| 100,000 | 1,000 | 5.770 | 4.922 | 203.19M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 192.30M | 222.52M | 1.00× | 3.78M | 4.26M | 1.00× | — |
| 2 | 332.50M | 408.16M | 1.83× | 3.64M | 3.88M | 0.91× | — |
| 4 | 480.91M | 732.23M | 3.29× | 3.94M | 4.12M | 0.97× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
