# DecayLinear benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 198.36M | 0.004 | 246.63M | nan | — | — |
| 10,000 | 0.036 | 281.20M | 0.033 | 307.00M | nan | — | — |
| 100,000 | 0.339 | 295.41M | 0.320 | 312.40M | nan | — | — |
| 1,000,000 | 3.621 | 276.18M | 3.207 | 311.82M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.332 ms**; native kernel **0.311 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.234 | 0.167 | 6.00M | nan | — | — |
| 100,000 | 10 | 0.920 | 0.515 | 19.40M | nan | — | — |
| 100,000 | 1,000 | 5.502 | 4.646 | 215.23M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 170.51M | 196.65M | 1.00× | 3.24M | 3.55M | 1.00× | — |
| 2 | 382.94M | 477.59M | 2.43× | 3.08M | 4.18M | 1.18× | — |
| 4 | 551.16M | 836.82M | 4.26× | 3.01M | 3.42M | 0.96× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
