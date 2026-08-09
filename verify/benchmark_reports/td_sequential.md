# TomDeMarkSequential benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 116.24M | 0.008 | 130.72M | nan | — | — |
| 10,000 | 0.067 | 149.47M | 0.064 | 155.69M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.012 ms**; native kernel **0.011 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.313 | 0.265 | 3.77M | nan | — | — |
| 1,500 | 10 | 1.015 | 0.573 | 17.46M | nan | — | — |
| 1,500 | 100 | 2.095 | 1.734 | 57.67M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 10.11M | 13.05M | 1.00× | 1.07M | 1.36M | 1.00× | — |
| 2 | 19.35M | 18.92M | 1.45× | 1.50M | 1.63M | 1.20× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
