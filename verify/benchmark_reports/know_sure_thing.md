# KnowSureThing benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.024 | 41.34M | 0.023 | 43.52M | nan | — | — |
| 10,000 | 0.215 | 46.50M | 0.208 | 48.08M | nan | — | — |
| 100,000 | 2.098 | 47.66M | 2.049 | 48.81M | nan | — | — |
| 1,000,000 | 21.624 | 46.24M | 20.328 | 49.19M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **2.096 ms**; native kernel **2.051 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.262 | 0.207 | 4.84M | nan | — | — |
| 100,000 | 10 | 1.136 | 0.807 | 12.39M | nan | — | — |
| 100,000 | 1,000 | 22.442 | 21.598 | 46.30M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 42.14M | 47.29M | 1.00× | 2.53M | 2.72M | 1.00× | — |
| 2 | 80.67M | 84.46M | 1.79× | 2.53M | 2.56M | 0.94× | — |
| 4 | 138.31M | 166.36M | 3.52× | 2.71M | 2.62M | 0.96× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
