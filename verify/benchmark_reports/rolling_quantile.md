# RollingQuantile benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.034 | 29.72M | 0.033 | 30.72M | nan | — | — |
| 10,000 | 0.409 | 24.47M | 0.383 | 26.12M | nan | — | — |
| 100,000 | 3.839 | 26.05M | 3.729 | 26.82M | nan | — | — |
| 1,000,000 | 38.091 | 26.25M | 37.816 | 26.44M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **3.729 ms**; native kernel **3.724 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.247 | 0.202 | 4.94M | nan | — | — |
| 100,000 | 10 | 1.255 | 0.894 | 11.18M | nan | — | — |
| 100,000 | 1,000 | 40.124 | 40.200 | 24.88M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 23.32M | 24.56M | 1.00× | 2.91M | 2.57M | 1.00× | — |
| 2 | 49.40M | 46.19M | 1.88× | 2.56M | 2.98M | 1.16× | — |
| 4 | 87.36M | 96.82M | 3.94× | 2.71M | 2.94M | 1.15× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
