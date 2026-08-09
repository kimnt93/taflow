# GarmanKlassYangZhang benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.029 | 34.15M | 0.029 | 34.53M | nan | — | — |
| 10,000 | 0.261 | 38.34M | 0.239 | 41.81M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.052 ms**; native kernel **0.045 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.452 | 0.348 | 2.87M | nan | — | — |
| 1,500 | 10 | 3.052 | 1.588 | 6.30M | nan | — | — |
| 1,500 | 100 | 6.422 | 4.838 | 20.67M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.59M | 8.60M | 1.00× | 877.93K | 963.72K | 1.00× | — |
| 2 | 12.80M | 13.99M | 1.63× | 994.77K | 1.02M | 1.06× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
