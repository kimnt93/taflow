# LaguerreRelativeStrengthIndex benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 90.99M | 0.010 | 96.14M | nan | — | — |
| 10,000 | 0.096 | 104.67M | 0.083 | 120.25M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.018 ms**; native kernel **0.017 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.444 | 0.338 | 2.96M | nan | — | — |
| 1,500 | 10 | 1.495 | 1.030 | 9.71M | nan | — | — |
| 1,500 | 100 | 4.163 | 3.805 | 26.28M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 8.80M | 13.88M | 1.00× | 1.15M | 954.47K | 1.00× | — |
| 2 | 10.54M | 12.87M | 0.93× | 808.60K | 453.92K | 0.48× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
