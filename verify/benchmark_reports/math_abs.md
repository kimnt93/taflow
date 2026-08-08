# MathAbs benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 319.41M | 0.002 | 510.49M | nan | — | — |
| 10,000 | 0.013 | 742.86M | 0.011 | 932.13M | nan | — | — |
| 100,000 | 0.173 | 578.19M | 0.141 | 711.03M | nan | — | — |
| 1,000,000 | 2.983 | 335.20M | 2.348 | 425.84M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.160 ms**; native kernel **0.131 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.211 | 0.142 | 7.04M | nan | — | — |
| 100,000 | 10 | 0.833 | 0.546 | 18.30M | nan | — | — |
| 100,000 | 1,000 | 5.569 | 2.723 | 367.28M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 394.72M | 531.12M | 1.00× | 3.54M | 3.12M | 1.00× | — |
| 2 | 458.39M | 653.43M | 1.23× | 3.21M | 3.51M | 1.13× | — |
| 4 | 413.30M | 700.29M | 1.32× | 3.10M | 3.33M | 1.07× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
