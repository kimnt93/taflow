# GarmanKlassYangZhang benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.024 | 41.55M | 0.022 | 44.86M | nan | — | — |
| 10,000 | 0.214 | 46.70M | 0.207 | 48.41M | nan | — | — |
| 100,000 | 2.044 | 48.92M | 2.006 | 49.84M | nan | — | — |
| 1,000,000 | 21.005 | 47.61M | 20.090 | 49.78M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **2.055 ms**; native kernel **2.026 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.332 | 0.276 | 3.63M | nan | — | — |
| 100,000 | 10 | 2.588 | 1.321 | 7.57M | nan | — | — |
| 100,000 | 1,000 | 23.966 | 41.233 | 24.25M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 40.65M | 42.63M | 1.00× | 1.82M | 1.99M | 1.00× | — |
| 2 | 79.34M | 61.91M | 1.45× | 2.42M | 2.62M | 1.32× | — |
| 4 | 132.37M | 119.88M | 2.81× | 2.19M | 2.24M | 1.13× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
