# OpeningRange benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 75.49M | 0.015 | 68.53M | nan | — | — |
| 10,000 | 0.077 | 129.66M | 0.070 | 143.52M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.018 ms**; native kernel **0.015 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.500 | 0.433 | 2.31M | nan | — | — |
| 1,500 | 10 | 1.875 | 1.198 | 8.35M | nan | — | — |
| 1,500 | 100 | 3.384 | 2.508 | 39.87M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 12.78M | 14.70M | 1.00× | 968.89K | 985.16K | 1.00× | — |
| 2 | 16.39M | 17.53M | 1.19× | 1.18M | 1.12M | 1.14× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
