# ExponentiallyWeightedVariance benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 124.41M | 0.007 | 140.78M | nan | — | — |
| 10,000 | 0.053 | 188.65M | 0.050 | 199.19M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.011 ms**; native kernel **0.010 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.275 | 0.207 | 4.82M | nan | — | — |
| 1,500 | 10 | 1.382 | 0.583 | 17.15M | nan | — | — |
| 1,500 | 100 | 3.331 | 1.700 | 58.81M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 8.17M | 11.43M | 1.00× | 1.15M | 1.56M | 1.00× | — |
| 2 | 13.96M | 23.76M | 2.08× | 1.76M | 1.63M | 1.04× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
