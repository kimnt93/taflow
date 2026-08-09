# Retracements benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.040 | 25.04M | 0.038 | 26.60M | nan | — | — |
| 10,000 | 0.410 | 24.41M | 0.394 | 25.36M | nan | — | — |
| 100,000 | 3.976 | 25.15M | 4.021 | 24.87M | nan | — | — |
| 1,000,000 | 55.356 | 18.06M | 40.183 | 24.89M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **3.941 ms**; native kernel **3.862 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.346 | 0.312 | 3.20M | nan | — | — |
| 100,000 | 10 | 2.525 | 1.281 | 7.81M | nan | — | — |
| 100,000 | 1,000 | 43.023 | 41.120 | 24.32M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 22.99M | 25.31M | 1.00× | 2.14M | 2.09M | 1.00× | — |
| 2 | 41.05M | 46.87M | 1.85× | 1.95M | 2.12M | 1.02× | — |
| 4 | 69.34M | 82.78M | 3.27× | 2.05M | 2.03M | 0.97× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
