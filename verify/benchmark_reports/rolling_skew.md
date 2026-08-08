# RollingSkew benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.048 | 20.83M | 0.046 | 21.89M | nan | — | — |
| 10,000 | 0.456 | 21.92M | 0.454 | 22.05M | nan | — | — |
| 100,000 | 4.623 | 21.63M | 4.426 | 22.60M | nan | — | — |
| 1,000,000 | 44.805 | 22.32M | 44.770 | 22.34M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **4.561 ms**; native kernel **4.356 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.253 | 0.216 | 4.64M | nan | — | — |
| 100,000 | 10 | 1.613 | 1.026 | 9.75M | nan | — | — |
| 100,000 | 1,000 | 54.688 | 47.422 | 21.09M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 21.46M | 22.66M | 1.00× | 2.15M | 2.55M | 1.00× | — |
| 2 | 40.02M | 41.82M | 1.85× | 2.38M | 2.91M | 1.14× | — |
| 4 | 68.04M | 64.92M | 2.86× | 2.45M | 2.80M | 1.10× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
