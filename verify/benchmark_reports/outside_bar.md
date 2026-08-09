# OutsideBar benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 208.33M | 0.003 | 287.57M | nan | — | — |
| 10,000 | 0.029 | 342.52M | 0.026 | 382.46M | nan | — | — |
| 100,000 | 0.268 | 373.16M | 0.242 | 412.65M | nan | — | — |
| 1,000,000 | 3.160 | 316.47M | 2.732 | 366.04M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.267 ms**; native kernel **0.242 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.263 | 0.187 | 5.36M | nan | — | — |
| 100,000 | 10 | 1.432 | 0.683 | 14.64M | nan | — | — |
| 100,000 | 1,000 | 4.974 | 3.894 | 256.82M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 240.13M | 298.28M | 1.00× | 3.31M | 3.56M | 1.00× | — |
| 2 | 394.80M | 489.60M | 1.64× | 3.08M | 3.61M | 1.01× | — |
| 4 | 536.15M | 855.52M | 2.87× | 3.19M | 3.41M | 0.96× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
