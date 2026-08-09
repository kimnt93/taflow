# LogReturn benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 88.40M | 0.011 | 93.52M | nan | — | — |
| 10,000 | 0.084 | 118.89M | 0.080 | 124.88M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.015 ms**; native kernel **0.014 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.233 | 0.165 | 6.05M | nan | — | — |
| 1,500 | 10 | 0.999 | 0.590 | 16.96M | nan | — | — |
| 1,500 | 100 | 2.500 | 1.954 | 51.17M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.25M | 15.98M | 1.00× | 1.42M | 1.36M | 1.00× | — |
| 2 | 18.39M | 13.48M | 0.84× | 1.37M | 1.74M | 1.28× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
