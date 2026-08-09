# CumulativeSumControlChart benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 193.82M | 0.004 | 232.71M | nan | — | — |
| 10,000 | 0.038 | 263.94M | 0.034 | 291.63M | nan | — | — |
| 100,000 | 0.346 | 289.21M | 0.324 | 309.07M | nan | — | — |
| 1,000,000 | 3.592 | 278.38M | 3.340 | 299.44M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.350 ms**; native kernel **0.322 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.191 | 0.140 | 7.15M | nan | — | — |
| 100,000 | 10 | 0.881 | 0.479 | 20.90M | nan | — | — |
| 100,000 | 1,000 | 5.152 | 4.485 | 222.97M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 202.82M | 207.25M | 1.00× | 3.01M | 2.96M | 1.00× | — |
| 2 | 349.32M | 405.61M | 1.96× | 3.43M | 3.87M | 1.31× | — |
| 4 | 500.76M | 794.66M | 3.83× | 3.70M | 3.83M | 1.29× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
