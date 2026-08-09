# MathDegrees benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 365.51M | 0.002 | 579.33M | nan | — | — |
| 10,000 | 0.016 | 620.98M | 0.013 | 750.13M | nan | — | — |
| 100,000 | 0.160 | 623.11M | 0.139 | 719.18M | nan | — | — |
| 1,000,000 | 2.805 | 356.47M | 2.130 | 469.39M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.155 ms**; native kernel **0.131 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.205 | 0.139 | 7.18M | nan | — | — |
| 100,000 | 10 | 0.837 | 0.586 | 17.08M | nan | — | — |
| 100,000 | 1,000 | 3.544 | 2.476 | 403.92M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 387.48M | 593.73M | 1.00× | 3.43M | 3.66M | 1.00× | — |
| 2 | 449.02M | 741.07M | 1.25× | 3.26M | 3.72M | 1.02× | — |
| 4 | 410.79M | 772.50M | 1.30× | 2.82M | 3.10M | 0.85× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
