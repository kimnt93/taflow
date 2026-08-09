# PremiumDiscount benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.022 | 45.27M | 0.020 | 49.31M | nan | — | — |
| 10,000 | 0.259 | 38.63M | 0.257 | 38.96M | nan | — | — |
| 100,000 | 2.666 | 37.52M | 2.551 | 39.20M | nan | — | — |
| 1,000,000 | 27.478 | 36.39M | 26.246 | 38.10M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **2.558 ms**; native kernel **2.518 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.264 | 0.203 | 4.92M | nan | — | — |
| 100,000 | 10 | 0.841 | 0.673 | 14.87M | nan | — | — |
| 100,000 | 1,000 | 30.590 | 26.262 | 38.08M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 36.01M | 37.94M | 1.00× | 2.39M | 2.67M | 1.00× | — |
| 2 | 34.89M | 35.97M | 0.95× | 2.57M | 2.82M | 1.06× | — |
| 4 | 33.97M | 33.73M | 0.89× | 2.56M | 2.69M | 1.00× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
