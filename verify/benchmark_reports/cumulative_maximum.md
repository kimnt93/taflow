# CumulativeMaximum benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 136.77M | 0.006 | 154.05M | nan | — | — |
| 10,000 | 0.049 | 202.66M | 0.045 | 220.74M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.009 ms**; native kernel **0.009 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.230 | 0.162 | 6.19M | nan | — | — |
| 1,500 | 10 | 0.934 | 0.536 | 18.65M | nan | — | — |
| 1,500 | 100 | 1.992 | 1.572 | 63.63M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 10.18M | 17.10M | 1.00× | 1.51M | 1.25M | 1.00× | — |
| 2 | 18.20M | 16.25M | 0.95× | 1.41M | 1.22M | 0.98× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
