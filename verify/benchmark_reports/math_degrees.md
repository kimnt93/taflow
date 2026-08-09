# MathDegrees benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 229.24M | 0.004 | 279.15M | nan | — | — |
| 10,000 | 0.015 | 653.67M | 0.013 | 790.15M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.005 ms**; native kernel **0.004 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.291 | 0.160 | 6.25M | nan | — | — |
| 1,500 | 10 | 1.072 | 0.562 | 17.78M | nan | — | — |
| 1,500 | 100 | 2.388 | 1.597 | 62.62M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 8.49M | 11.73M | 1.00× | 1.38M | 1.47M | 1.00× | — |
| 2 | 14.57M | 16.65M | 1.42× | 1.46M | 1.71M | 1.17× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
