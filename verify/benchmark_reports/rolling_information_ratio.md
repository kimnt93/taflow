# RollingInformationRatio benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.038 | 26.17M | 0.038 | 26.33M | nan | — | — |
| 10,000 | 0.340 | 29.43M | 0.338 | 29.57M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.054 ms**; native kernel **0.051 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.328 | 0.240 | 4.17M | nan | — | — |
| 1,500 | 10 | 1.775 | 1.071 | 9.34M | nan | — | — |
| 1,500 | 100 | 5.302 | 5.817 | 17.19M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 10.34M | 6.11M | 1.00× | 683.46K | 1.09M | 1.00× | — |
| 2 | 13.26M | 14.11M | 2.31× | 1.38M | 1.47M | 1.35× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
