# MathRadians benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 368.49M | 0.002 | 593.59M | nan | — | — |
| 10,000 | 0.017 | 603.61M | 0.014 | 736.30M | nan | — | — |
| 100,000 | 0.157 | 637.49M | 0.131 | 762.17M | nan | — | — |
| 1,000,000 | 2.626 | 380.83M | 2.148 | 465.60M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.160 ms**; native kernel **0.130 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.209 | 0.133 | 7.54M | nan | — | — |
| 100,000 | 10 | 0.817 | 0.471 | 21.22M | nan | — | — |
| 100,000 | 1,000 | 3.277 | 5.170 | 193.42M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 354.45M | 521.72M | 1.00× | 3.21M | 4.03M | 1.00× | — |
| 2 | 502.67M | 962.81M | 1.85× | 3.40M | 4.19M | 1.04× | — |
| 4 | 429.28M | 843.23M | 1.62× | 3.12M | 3.66M | 0.91× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
