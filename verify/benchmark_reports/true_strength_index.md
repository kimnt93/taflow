# TrueStrengthIndex benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 132.75M | 0.007 | 152.01M | nan | — | — |
| 10,000 | 0.056 | 178.56M | 0.053 | 187.79M | nan | — | — |
| 100,000 | 0.560 | 178.48M | 0.526 | 190.03M | nan | — | — |
| 1,000,000 | 5.818 | 171.89M | 5.395 | 185.35M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.549 ms**; native kernel **0.529 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.224 | 0.154 | 6.50M | nan | — | — |
| 100,000 | 10 | 0.903 | 0.537 | 18.62M | nan | — | — |
| 100,000 | 1,000 | 7.666 | 7.070 | 141.45M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 123.99M | 135.99M | 1.00× | 2.85M | 3.08M | 1.00× | — |
| 2 | 261.03M | 312.46M | 2.30× | 3.85M | 3.99M | 1.30× | — |
| 4 | 369.32M | 558.03M | 4.10× | 3.69M | 3.88M | 1.26× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
