# EaseOfMovement benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 185.96M | 0.004 | 265.38M | nan | — | — |
| 10,000 | 0.031 | 326.97M | 0.027 | 370.32M | nan | — | — |
| 100,000 | 0.278 | 360.25M | 0.255 | 392.90M | nan | — | — |
| 1,000,000 | 3.309 | 302.22M | 2.964 | 337.39M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.278 ms**; native kernel **0.252 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.274 | 0.213 | 4.69M | nan | — | — |
| 100,000 | 10 | 2.062 | 1.015 | 9.85M | nan | — | — |
| 100,000 | 1,000 | 5.554 | 4.283 | 233.46M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 207.87M | 268.64M | 1.00× | 2.96M | 3.12M | 1.00× | — |
| 2 | 371.40M | 547.18M | 2.04× | 2.72M | 2.89M | 0.93× | — |
| 4 | 529.77M | 900.61M | 3.35× | 2.97M | 3.03M | 0.97× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
