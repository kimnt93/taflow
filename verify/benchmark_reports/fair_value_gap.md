# FairValueGap benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 62.86M | 0.013 | 77.40M | nan | — | — |
| 10,000 | 0.119 | 84.10M | 0.104 | 96.22M | nan | — | — |
| 100,000 | 1.126 | 88.78M | 1.030 | 97.05M | nan | — | — |
| 1,000,000 | 27.556 | 36.29M | 18.805 | 53.18M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.405 ms**; native kernel **1.021 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.426 | 0.324 | 3.09M | nan | — | — |
| 100,000 | 10 | 2.530 | 1.234 | 8.10M | nan | — | — |
| 100,000 | 1,000 | 16.865 | 11.743 | 85.16M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 64.25M | 74.96M | 1.00× | 1.79M | 1.73M | 1.00× | — |
| 2 | 86.99M | 145.57M | 1.94× | 2.19M | 2.07M | 1.19× | — |
| 4 | 126.66M | 217.34M | 2.90× | 2.03M | 1.86M | 1.08× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
