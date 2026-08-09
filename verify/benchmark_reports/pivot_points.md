# PivotPoints benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.025 | 39.83M | 0.015 | 66.86M | nan | — | — |
| 10,000 | 0.112 | 89.47M | 0.099 | 101.35M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.023 ms**; native kernel **0.019 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.473 | 0.461 | 2.17M | nan | — | — |
| 1,500 | 10 | 1.834 | 1.188 | 8.42M | nan | — | — |
| 1,500 | 100 | 3.617 | 2.887 | 34.64M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.10M | 7.58M | 1.00× | 827.76K | 837.80K | 1.00× | — |
| 2 | 12.95M | 13.76M | 1.82× | 1.00M | 1.02M | 1.22× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
