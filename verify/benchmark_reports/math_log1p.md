# MathLog1p benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 85.12M | 0.010 | 96.34M | nan | — | — |
| 10,000 | 0.091 | 109.98M | 0.082 | 122.65M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.015 ms**; native kernel **0.014 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.311 | 0.171 | 5.84M | nan | — | — |
| 1,500 | 10 | 1.139 | 0.644 | 15.52M | nan | — | — |
| 1,500 | 100 | 3.178 | 2.369 | 42.21M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 9.57M | 15.93M | 1.00× | 1.37M | 1.52M | 1.00× | — |
| 2 | 20.19M | 19.67M | 1.23× | 1.29M | 1.71M | 1.13× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
