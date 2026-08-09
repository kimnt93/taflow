# MathAbs benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 367.17M | 0.002 | 563.75M | nan | — | — |
| 10,000 | 0.017 | 600.34M | 0.014 | 702.78M | nan | — | — |
| 100,000 | 0.159 | 627.60M | 0.135 | 740.59M | nan | — | — |
| 1,000,000 | 2.625 | 380.91M | 2.080 | 480.80M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.158 ms**; native kernel **0.135 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.210 | 0.151 | 6.64M | nan | — | — |
| 100,000 | 10 | 0.860 | 0.488 | 20.49M | nan | — | — |
| 100,000 | 1,000 | 3.209 | 2.483 | 402.81M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 343.19M | 590.93M | 1.00× | 3.05M | 3.68M | 1.00× | — |
| 2 | 535.55M | 930.24M | 1.57× | 3.30M | 3.38M | 0.92× | — |
| 4 | 435.42M | 834.34M | 1.41× | 3.07M | 3.39M | 0.92× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
