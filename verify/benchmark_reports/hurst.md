# Hurst benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.056 | 17.71M | 0.055 | 18.16M | nan | — | — |
| 10,000 | 0.539 | 18.55M | 0.537 | 18.64M | nan | — | — |
| 100,000 | 5.299 | 18.87M | 5.343 | 18.72M | nan | — | — |
| 1,000,000 | 53.702 | 18.62M | 53.120 | 18.83M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **5.502 ms**; native kernel **5.601 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.247 | 0.195 | 5.14M | nan | — | — |
| 100,000 | 10 | 1.338 | 0.997 | 10.03M | nan | — | — |
| 100,000 | 1,000 | 57.079 | 52.937 | 18.89M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 17.15M | 17.74M | 1.00× | 2.33M | 2.67M | 1.00× | — |
| 2 | 32.82M | 33.71M | 1.90× | 2.86M | 2.59M | 0.97× | — |
| 4 | 52.51M | 47.53M | 2.68× | 2.45M | 2.44M | 0.91× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
