# AverageDirectionalIndexRating benchmark (`ADXR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 78.44M | 0.011 | 92.73M | 0.039 | 3.06× | 3.61× |
| 10,000 | 0.115 | 86.65M | 0.101 | 99.18M | 0.143 | 1.24× | 1.42× |
| 100,000 | 1.115 | 89.72M | 0.914 | 109.45M | 0.958 | 0.86× | 1.05× |
| 1,000,000 | 11.768 | 84.97M | 9.218 | 108.48M | 9.976 | 0.85× | 1.08× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.108 ms**; native kernel **0.893 ms**; TA-Lib 0.986 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.309 | 0.260 | 3.85M | 949.074 | 3656.35× | 117.58× |
| 100,000 | 10 | 1.285 | 0.992 | 10.08M | 962.786 | 970.56× | 31.80× |
| 100,000 | 1,000 | 12.758 | 10.967 | 91.18M | 1008.198 | 91.93× | 3.64× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 90.04M | 92.20M | 1.00× | 2.32M | 2.39M | 1.00× | 86.08M |
| 2 | 164.38M | 177.43M | 1.92× | 2.55M | 2.90M | 1.21× | 87.50M |
| 4 | 254.99M | 318.49M | 3.45× | 2.27M | 2.55M | 1.07× | 83.36M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
