# ForceIndex benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 181.50M | 0.004 | 256.70M | nan | — | — |
| 10,000 | 0.034 | 296.34M | 0.030 | 332.73M | nan | — | — |
| 100,000 | 0.308 | 324.41M | 0.287 | 348.30M | nan | — | — |
| 1,000,000 | 3.671 | 272.40M | 3.207 | 311.80M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.302 ms**; native kernel **0.275 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.263 | 0.202 | 4.95M | nan | — | — |
| 100,000 | 10 | 1.478 | 0.784 | 12.76M | nan | — | — |
| 100,000 | 1,000 | 5.480 | 4.385 | 228.06M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 225.56M | 296.96M | 1.00× | 2.74M | 2.84M | 1.00× | — |
| 2 | 377.88M | 558.32M | 1.88× | 2.96M | 3.01M | 1.06× | — |
| 4 | 532.56M | 909.38M | 3.06× | 3.11M | 3.38M | 1.19× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
