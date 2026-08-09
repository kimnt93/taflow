# TimeSeriesRank benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.022 | 45.76M | 0.021 | 48.65M | nan | — | — |
| 10,000 | 0.183 | 54.66M | 0.180 | 55.64M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.031 ms**; native kernel **0.029 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.330 | 0.200 | 4.99M | nan | — | — |
| 1,500 | 10 | 1.282 | 0.768 | 13.03M | nan | — | — |
| 1,500 | 100 | 3.755 | 3.068 | 32.60M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.83M | 7.51M | 1.00× | 1.11M | 1.19M | 1.00× | — |
| 2 | 14.94M | 19.80M | 2.64× | 1.38M | 1.54M | 1.30× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
