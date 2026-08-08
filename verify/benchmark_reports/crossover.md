# Crossover benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 151.95M | 0.004 | 233.82M | nan | — | — |
| 10,000 | 0.037 | 267.55M | 0.033 | 299.39M | nan | — | — |
| 100,000 | 0.337 | 296.42M | 0.311 | 321.62M | nan | — | — |
| 1,000,000 | 4.060 | 246.33M | 3.394 | 294.63M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.330 ms**; native kernel **0.312 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.272 | 0.194 | 5.16M | nan | — | — |
| 100,000 | 10 | 1.584 | 0.799 | 12.51M | nan | — | — |
| 100,000 | 1,000 | 6.073 | 4.949 | 202.07M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 201.11M | 214.75M | 1.00× | 3.13M | 3.16M | 1.00× | — |
| 2 | 330.06M | 476.74M | 2.22× | 2.98M | 3.03M | 0.96× | — |
| 4 | 517.98M | 729.35M | 3.40× | 3.18M | 3.42M | 1.08× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
