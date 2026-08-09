# ExponentiallyWeightedCorrelation benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 131.06M | 0.006 | 157.68M | nan | — | — |
| 10,000 | 0.055 | 180.56M | 0.052 | 191.96M | nan | — | — |
| 100,000 | 0.532 | 187.94M | 0.521 | 191.84M | nan | — | — |
| 1,000,000 | 5.667 | 176.45M | 5.132 | 194.85M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.525 ms**; native kernel **0.523 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.249 | 0.191 | 5.23M | nan | — | — |
| 100,000 | 10 | 1.456 | 0.848 | 11.79M | nan | — | — |
| 100,000 | 1,000 | 7.936 | 6.759 | 147.95M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 151.66M | 173.67M | 1.00× | 3.38M | 3.30M | 1.00× | — |
| 2 | 268.49M | 306.01M | 1.76× | 3.13M | 3.31M | 1.00× | — |
| 4 | 360.38M | 476.92M | 2.75× | 3.06M | 3.24M | 0.98× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
