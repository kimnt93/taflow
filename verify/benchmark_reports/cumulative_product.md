# CumulativeProduct benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 223.99M | 0.003 | 296.94M | nan | — | — |
| 10,000 | 0.030 | 336.57M | 0.026 | 391.49M | nan | — | — |
| 100,000 | 0.259 | 386.46M | 0.234 | 426.88M | nan | — | — |
| 1,000,000 | 2.932 | 341.09M | 2.934 | 340.78M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.272 ms**; native kernel **0.250 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.211 | 0.164 | 6.09M | nan | — | — |
| 100,000 | 10 | 0.895 | 0.500 | 19.98M | nan | — | — |
| 100,000 | 1,000 | 4.272 | 3.458 | 289.16M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 258.01M | 288.78M | 1.00× | 3.72M | 3.86M | 1.00× | — |
| 2 | 406.46M | 600.53M | 2.08× | 3.68M | 3.80M | 0.99× | — |
| 4 | 590.42M | 882.39M | 3.06× | 3.75M | 4.12M | 1.07× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
