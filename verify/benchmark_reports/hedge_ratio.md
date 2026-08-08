# HedgeRatio benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.040 | 25.09M | 0.040 | 25.07M | nan | — | — |
| 10,000 | 0.390 | 25.64M | 0.382 | 26.19M | nan | — | — |
| 100,000 | 3.777 | 26.48M | 3.800 | 26.31M | nan | — | — |
| 1,000,000 | 42.237 | 23.68M | 39.431 | 25.36M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **3.939 ms**; native kernel **3.720 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.271 | 0.215 | 4.65M | nan | — | — |
| 100,000 | 10 | 1.766 | 1.078 | 9.27M | nan | — | — |
| 100,000 | 1,000 | 39.493 | 51.250 | 19.51M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 24.96M | 25.40M | 1.00× | 2.82M | 2.33M | 1.00× | — |
| 2 | 46.36M | 44.77M | 1.76× | 2.19M | 2.82M | 1.21× | — |
| 4 | 82.40M | 83.14M | 3.27× | 2.45M | 2.53M | 1.09× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
