# Supertrend benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.024 | 42.04M | 0.023 | 43.47M | nan | — | — |
| 10,000 | 0.244 | 40.98M | 0.218 | 45.94M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.035 ms**; native kernel **0.033 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.442 | 0.333 | 3.00M | nan | — | — |
| 1,500 | 10 | 2.348 | 1.237 | 8.08M | nan | — | — |
| 1,500 | 100 | 5.009 | 3.655 | 27.36M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 9.63M | 9.98M | 1.00× | 773.26K | 1.13M | 1.00× | — |
| 2 | 13.96M | 15.32M | 1.54× | 1.22M | 1.24M | 1.09× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
