# VolumePriceTrend benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 210.21M | 0.003 | 285.79M | nan | — | — |
| 10,000 | 0.030 | 337.88M | 0.026 | 385.61M | nan | — | — |
| 100,000 | 0.262 | 381.90M | 0.241 | 415.75M | nan | — | — |
| 1,000,000 | 3.061 | 326.65M | 2.683 | 372.68M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.263 ms**; native kernel **0.237 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.234 | 0.178 | 5.62M | nan | — | — |
| 100,000 | 10 | 1.374 | 0.715 | 13.98M | nan | — | — |
| 100,000 | 1,000 | 5.493 | 4.115 | 243.04M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 217.72M | 283.59M | 1.00× | 3.07M | 2.69M | 1.00× | — |
| 2 | 435.45M | 551.12M | 1.94× | 3.28M | 3.46M | 1.29× | — |
| 4 | 567.53M | 891.91M | 3.15× | 3.36M | 3.39M | 1.26× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
