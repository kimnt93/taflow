# GarmanKlassYangZhang benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.026 | 38.89M | 0.024 | 42.19M | nan | — | — |
| 10,000 | 0.230 | 43.41M | 0.221 | 45.19M | nan | — | — |
| 100,000 | 2.199 | 45.48M | 2.174 | 46.00M | nan | — | — |
| 1,000,000 | 21.936 | 45.59M | 21.967 | 45.52M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **2.234 ms**; native kernel **2.114 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.349 | 0.283 | 3.54M | nan | — | — |
| 100,000 | 10 | 2.554 | 1.336 | 7.48M | nan | — | — |
| 100,000 | 1,000 | 32.832 | 30.798 | 32.47M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 41.89M | 43.90M | 1.00× | 2.10M | 2.57M | 1.00× | — |
| 2 | 79.47M | 84.40M | 1.92× | 2.17M | 2.29M | 0.89× | — |
| 4 | 125.14M | 130.05M | 2.96× | 2.01M | 2.31M | 0.90× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
