# Sessions benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 86.62M | 0.009 | 109.37M | nan | — | — |
| 10,000 | 0.084 | 118.47M | 0.076 | 131.56M | nan | — | — |
| 100,000 | 0.755 | 132.41M | 0.688 | 145.36M | nan | — | — |
| 1,000,000 | 19.682 | 50.81M | 7.726 | 129.44M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.761 ms**; native kernel **0.683 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.474 | 0.345 | 2.90M | nan | — | — |
| 100,000 | 10 | 1.963 | 1.063 | 9.41M | nan | — | — |
| 100,000 | 1,000 | 9.960 | 8.594 | 116.36M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 86.98M | 106.01M | 1.00× | 1.94M | 1.71M | 1.00× | — |
| 2 | 147.34M | 206.16M | 1.94× | 1.91M | 2.03M | 1.19× | — |
| 4 | 193.30M | 323.61M | 3.05× | 1.94M | 1.94M | 1.13× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
