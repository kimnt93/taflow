# FisherTransform benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.049 | 20.33M | 0.040 | 25.16M | nan | — | — |
| 10,000 | 0.487 | 20.52M | 0.390 | 25.63M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.069 ms**; native kernel **0.061 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.409 | 0.367 | 2.73M | nan | — | — |
| 1,500 | 10 | 1.926 | 1.138 | 8.79M | nan | — | — |
| 1,500 | 100 | 6.556 | 5.121 | 19.53M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 6.44M | 7.72M | 1.00× | 1.02M | 1.29M | 1.00× | — |
| 2 | 12.60M | 15.70M | 2.04× | 1.34M | 1.42M | 1.10× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
