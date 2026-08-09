# DecayLinear benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 140.09M | 0.006 | 168.49M | nan | — | — |
| 10,000 | 0.045 | 222.70M | 0.038 | 265.65M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.009 ms**; native kernel **0.007 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.361 | 0.234 | 4.27M | nan | — | — |
| 1,500 | 10 | 1.204 | 0.607 | 16.46M | nan | — | — |
| 1,500 | 100 | 2.627 | 1.928 | 51.87M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 11.03M | 17.23M | 1.00× | 1.21M | 1.42M | 1.00× | — |
| 2 | 17.14M | 19.05M | 1.11× | 1.26M | 1.53M | 1.08× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
