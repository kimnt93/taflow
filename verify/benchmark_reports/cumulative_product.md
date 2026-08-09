# CumulativeProduct benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 191.29M | 0.004 | 229.61M | nan | — | — |
| 10,000 | 0.026 | 382.99M | 0.024 | 414.32M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.006 ms**; native kernel **0.006 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.229 | 0.161 | 6.20M | nan | — | — |
| 1,500 | 10 | 0.919 | 0.544 | 18.39M | nan | — | — |
| 1,500 | 100 | 1.823 | 1.310 | 76.32M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 11.71M | 13.57M | 1.00× | 1.44M | 1.48M | 1.00× | — |
| 2 | 20.89M | 24.17M | 1.78× | 1.51M | 1.77M | 1.19× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
