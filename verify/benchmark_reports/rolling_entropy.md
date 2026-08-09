# RollingEntropy benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.713 | 1.40M | 0.710 | 1.41M | nan | — | — |
| 10,000 | 7.250 | 1.38M | 7.447 | 1.34M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **1.097 ms**; native kernel **1.096 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 1.182 | 0.855 | 1.17M | nan | — | — |
| 1,500 | 10 | 7.896 | 8.733 | 1.15M | nan | — | — |
| 1,500 | 100 | 74.872 | 71.243 | 1.40M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 1.01M | 1.12M | 1.00× | 476.61K | 400.81K | 1.00× | — |
| 2 | 2.09M | 2.04M | 1.82× | 646.46K | 690.04K | 1.72× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
