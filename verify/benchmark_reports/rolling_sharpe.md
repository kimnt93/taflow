# RollingSharpe benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.036 | 27.86M | 0.032 | 31.28M | nan | — | — |
| 10,000 | 0.316 | 31.70M | 0.302 | 33.13M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.052 ms**; native kernel **0.047 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.265 | 0.216 | 4.64M | nan | — | — |
| 1,500 | 10 | 1.221 | 0.816 | 12.26M | nan | — | — |
| 1,500 | 100 | 4.493 | 4.033 | 24.79M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 10.98M | 12.36M | 1.00× | 1.40M | 1.48M | 1.00× | — |
| 2 | 14.31M | 16.56M | 1.34× | 1.37M | 1.63M | 1.10× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
