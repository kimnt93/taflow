# Squeeze benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.031 | 32.22M | 0.028 | 36.14M | nan | — | — |
| 10,000 | 0.261 | 38.36M | 0.246 | 40.72M | nan | — | — |
| 100,000 | 2.596 | 38.51M | 2.488 | 40.20M | nan | — | — |
| 1,000,000 | 41.924 | 23.85M | 33.518 | 29.83M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **2.603 ms**; native kernel **2.466 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.358 | 0.282 | 3.55M | nan | — | — |
| 100,000 | 10 | 2.318 | 1.200 | 8.34M | nan | — | — |
| 100,000 | 1,000 | 31.190 | 25.866 | 38.66M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 35.26M | 38.72M | 1.00× | 2.29M | 2.19M | 1.00× | — |
| 2 | 61.20M | 72.57M | 1.87× | 2.25M | 2.12M | 0.97× | — |
| 4 | 88.50M | 129.15M | 3.33× | 2.08M | 2.07M | 0.94× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
