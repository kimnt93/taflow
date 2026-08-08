# LowerLow benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 176.78M | 0.004 | 246.06M | nan | — | — |
| 10,000 | 0.036 | 281.33M | 0.033 | 299.53M | nan | — | — |
| 100,000 | 0.307 | 326.08M | 0.294 | 340.46M | nan | — | — |
| 1,000,000 | 3.714 | 269.23M | 3.219 | 310.66M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.307 ms**; native kernel **0.278 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.248 | 0.199 | 5.03M | nan | — | — |
| 100,000 | 10 | 1.498 | 0.838 | 11.93M | nan | — | — |
| 100,000 | 1,000 | 5.437 | 4.313 | 231.88M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 235.79M | 287.94M | 1.00× | 3.03M | 3.33M | 1.00× | — |
| 2 | 424.25M | 551.45M | 1.92× | 3.29M | 3.52M | 1.06× | — |
| 4 | 535.97M | 877.90M | 3.05× | 3.23M | 3.30M | 0.99× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
