# MassIndex benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 65.18M | 0.010 | 101.71M | nan | — | — |
| 10,000 | 0.070 | 143.65M | 0.066 | 150.46M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.014 ms**; native kernel **0.013 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.300 | 0.220 | 4.55M | nan | — | — |
| 1,500 | 10 | 1.667 | 0.897 | 11.15M | nan | — | — |
| 1,500 | 100 | 3.136 | 2.235 | 44.74M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 9.13M | 15.46M | 1.00× | 1.31M | 1.42M | 1.00× | — |
| 2 | 14.17M | 13.55M | 0.88× | 999.88K | 1.23M | 0.87× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
