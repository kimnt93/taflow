# FisherTransform benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.035 | 28.62M | 0.033 | 30.16M | nan | — | — |
| 10,000 | 0.364 | 27.50M | 0.358 | 27.91M | nan | — | — |
| 100,000 | 3.742 | 26.73M | 3.582 | 27.92M | nan | — | — |
| 1,000,000 | 37.032 | 27.00M | 36.734 | 27.22M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **3.708 ms**; native kernel **3.699 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.272 | 0.220 | 4.55M | nan | — | — |
| 100,000 | 10 | 1.831 | 1.062 | 9.42M | nan | — | — |
| 100,000 | 1,000 | 38.231 | 36.969 | 27.05M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 24.70M | 25.86M | 1.00× | 2.29M | 2.68M | 1.00× | — |
| 2 | 44.16M | 45.81M | 1.77× | 2.49M | 2.62M | 0.98× | — |
| 4 | 72.09M | 93.66M | 3.62× | 2.46M | 2.59M | 0.97× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
