# AverageDailyDollarValue benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 130.33M | 0.006 | 156.50M | nan | — | — |
| 10,000 | 0.052 | 191.79M | 0.048 | 207.41M | nan | — | — |
| 100,000 | 0.482 | 207.68M | 0.464 | 215.66M | nan | — | — |
| 1,000,000 | 5.320 | 187.96M | 4.995 | 200.19M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.481 ms**; native kernel **0.468 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.283 | 0.201 | 4.97M | nan | — | — |
| 100,000 | 10 | 1.473 | 0.779 | 12.84M | nan | — | — |
| 100,000 | 1,000 | 9.143 | 8.599 | 116.30M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 161.99M | 195.91M | 1.00× | 3.46M | 3.64M | 1.00× | — |
| 2 | 285.02M | 322.22M | 1.64× | 3.15M | 3.56M | 0.98× | — |
| 4 | 396.61M | 597.58M | 3.05× | 3.23M | 3.31M | 0.91× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
