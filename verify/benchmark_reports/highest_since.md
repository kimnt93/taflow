# HighestSince benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 132.90M | 0.006 | 154.07M | nan | — | — |
| 10,000 | 0.040 | 251.68M | 0.037 | 273.69M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.009 ms**; native kernel **0.008 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.341 | 0.314 | 3.19M | nan | — | — |
| 1,500 | 10 | 1.355 | 0.761 | 13.14M | nan | — | — |
| 1,500 | 100 | 2.421 | 1.717 | 58.25M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 9.81M | 15.64M | 1.00× | 1.06M | 1.17M | 1.00× | — |
| 2 | 17.29M | 20.10M | 1.29× | 1.20M | 1.12M | 0.95× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
