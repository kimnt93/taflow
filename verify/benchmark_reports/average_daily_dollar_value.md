# AverageDailyDollarValue benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 131.44M | 0.006 | 159.08M | nan | — | — |
| 10,000 | 0.051 | 195.53M | 0.048 | 207.17M | nan | — | — |
| 100,000 | 0.474 | 210.95M | 0.446 | 224.46M | nan | — | — |
| 1,000,000 | 5.130 | 194.93M | 4.773 | 209.51M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.472 ms**; native kernel **0.446 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.264 | 0.182 | 5.49M | nan | — | — |
| 100,000 | 10 | 1.418 | 0.746 | 13.40M | nan | — | — |
| 100,000 | 1,000 | 7.195 | 6.129 | 163.17M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 139.61M | 176.58M | 1.00× | 2.84M | 3.29M | 1.00× | — |
| 2 | 271.89M | 338.08M | 1.91× | 3.16M | 3.54M | 1.08× | — |
| 4 | 408.79M | 626.68M | 3.55× | 3.37M | 3.39M | 1.03× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
