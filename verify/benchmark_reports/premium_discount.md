# PremiumDiscount benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 47.12M | 0.020 | 50.24M | nan | — | — |
| 10,000 | 0.275 | 36.41M | 0.270 | 37.02M | nan | — | — |
| 100,000 | 2.714 | 36.85M | 2.644 | 37.82M | nan | — | — |
| 1,000,000 | 27.881 | 35.87M | 27.621 | 36.20M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **2.727 ms**; native kernel **2.646 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.269 | 0.212 | 4.73M | nan | — | — |
| 100,000 | 10 | 0.873 | 0.684 | 14.63M | nan | — | — |
| 100,000 | 1,000 | 28.527 | 28.074 | 35.62M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 32.23M | 34.10M | 1.00× | 2.19M | 2.72M | 1.00× | — |
| 2 | 33.36M | 35.13M | 1.03× | 2.73M | 2.64M | 0.97× | — |
| 4 | 33.55M | 33.70M | 0.99× | 2.46M | 2.70M | 1.00× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
