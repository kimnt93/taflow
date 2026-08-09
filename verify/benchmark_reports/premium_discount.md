# PremiumDiscount benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.024 | 42.09M | 0.025 | 39.73M | nan | — | — |
| 10,000 | 0.277 | 36.08M | 0.274 | 36.46M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.038 ms**; native kernel **0.037 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.319 | 0.244 | 4.10M | nan | — | — |
| 1,500 | 10 | 0.993 | 0.747 | 13.39M | nan | — | — |
| 1,500 | 100 | 4.130 | 3.653 | 27.37M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 9.19M | 11.59M | 1.00× | 1.04M | 1.25M | 1.00× | — |
| 2 | 12.26M | 13.47M | 1.16× | 1.44M | 1.48M | 1.19× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
