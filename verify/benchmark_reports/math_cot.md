# MathCot benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 53.81M | 0.017 | 57.75M | nan | — | — |
| 10,000 | 0.210 | 47.67M | 0.206 | 48.56M | nan | — | — |
| 100,000 | 2.127 | 47.02M | 2.242 | 44.61M | nan | — | — |
| 1,000,000 | 22.885 | 43.70M | 21.666 | 46.15M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **2.148 ms**; native kernel **2.149 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.244 | 0.193 | 5.19M | nan | — | — |
| 100,000 | 10 | 1.288 | 0.721 | 13.86M | nan | — | — |
| 100,000 | 1,000 | 30.140 | 21.461 | 46.60M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 43.61M | 40.83M | 1.00× | 2.32M | 2.60M | 1.00× | — |
| 2 | 79.27M | 83.63M | 2.05× | 2.30M | 2.73M | 1.05× | — |
| 4 | 122.20M | 155.74M | 3.81× | 2.48M | 2.51M | 0.97× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
