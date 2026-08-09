# HedgeRatio benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.038 | 25.99M | 0.037 | 27.39M | nan | — | — |
| 10,000 | 0.365 | 27.43M | 0.354 | 28.22M | nan | — | — |
| 100,000 | 3.627 | 27.57M | 3.475 | 28.78M | nan | — | — |
| 1,000,000 | 35.862 | 27.88M | 34.955 | 28.61M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **3.547 ms**; native kernel **3.517 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.260 | 0.207 | 4.83M | nan | — | — |
| 100,000 | 10 | 1.699 | 1.009 | 9.91M | nan | — | — |
| 100,000 | 1,000 | 39.879 | 35.857 | 27.89M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 26.06M | 25.94M | 1.00× | 2.49M | 2.72M | 1.00× | — |
| 2 | 48.19M | 43.74M | 1.69× | 2.41M | 2.57M | 0.94× | — |
| 4 | 90.00M | 99.26M | 3.83× | 2.55M | 2.71M | 0.99× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
