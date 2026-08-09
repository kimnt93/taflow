# RollingZScore benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.031 | 32.19M | 0.031 | 32.27M | nan | — | — |
| 10,000 | 0.280 | 35.74M | 0.283 | 35.32M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.044 ms**; native kernel **0.044 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.257 | 0.190 | 5.26M | nan | — | — |
| 1,500 | 10 | 2.743 | 0.776 | 12.88M | nan | — | — |
| 1,500 | 100 | 4.347 | 3.860 | 25.90M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.73M | 8.70M | 1.00× | 1.42M | 1.22M | 1.00× | — |
| 2 | 14.36M | 15.72M | 1.81× | 1.41M | 1.31M | 1.07× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
