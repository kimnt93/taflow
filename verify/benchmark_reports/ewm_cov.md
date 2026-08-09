# ExponentiallyWeightedCovariance benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 145.02M | 0.006 | 174.72M | nan | — | — |
| 10,000 | 0.050 | 199.45M | 0.046 | 218.32M | nan | — | — |
| 100,000 | 0.471 | 212.48M | 0.437 | 229.04M | nan | — | — |
| 1,000,000 | 4.931 | 202.79M | 4.560 | 219.32M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.466 ms**; native kernel **0.466 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.228 | 0.182 | 5.49M | nan | — | — |
| 100,000 | 10 | 1.521 | 0.728 | 13.73M | nan | — | — |
| 100,000 | 1,000 | 7.290 | 7.920 | 126.26M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 114.68M | 187.48M | 1.00× | 3.41M | 3.58M | 1.00× | — |
| 2 | 288.52M | 315.01M | 1.68× | 3.06M | 3.34M | 0.93× | — |
| 4 | 350.40M | 429.35M | 2.29× | 3.30M | 3.33M | 0.93× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
