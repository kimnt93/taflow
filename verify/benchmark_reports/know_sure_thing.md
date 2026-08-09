# KnowSureThing benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 57.22M | 0.016 | 61.57M | nan | — | — |
| 10,000 | 0.154 | 65.08M | 0.149 | 66.98M | nan | — | — |
| 100,000 | 1.490 | 67.10M | 1.450 | 68.96M | nan | — | — |
| 1,000,000 | 24.639 | 40.59M | 14.537 | 68.79M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.496 ms**; native kernel **1.457 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.302 | 0.276 | 3.62M | nan | — | — |
| 100,000 | 10 | 1.495 | 1.104 | 9.05M | nan | — | — |
| 100,000 | 1,000 | 61.859 | 62.748 | 15.94M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 56.96M | 59.55M | 1.00× | 1.96M | 1.86M | 1.00× | — |
| 2 | 99.60M | 119.19M | 2.00× | 2.05M | 2.00M | 1.07× | — |
| 4 | 168.23M | 232.61M | 3.91× | 2.13M | 2.13M | 1.15× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
