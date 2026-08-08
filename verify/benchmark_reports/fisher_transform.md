# FisherTransform benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.040 | 24.85M | 0.036 | 27.94M | nan | — | — |
| 10,000 | 0.383 | 26.10M | 0.401 | 24.93M | nan | — | — |
| 100,000 | 3.946 | 25.34M | 3.842 | 26.03M | nan | — | — |
| 1,000,000 | 37.793 | 26.46M | 37.577 | 26.61M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **3.741 ms**; native kernel **3.730 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.313 | 0.269 | 3.71M | nan | — | — |
| 100,000 | 10 | 1.772 | 1.083 | 9.23M | nan | — | — |
| 100,000 | 1,000 | 38.735 | 37.083 | 26.97M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 24.74M | 22.69M | 1.00× | 2.39M | 2.49M | 1.00× | — |
| 2 | 46.47M | 49.39M | 2.18× | 2.24M | 2.53M | 1.02× | — |
| 4 | 75.85M | 69.85M | 3.08× | 2.39M | 2.61M | 1.05× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
