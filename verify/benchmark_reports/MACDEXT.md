# MovingAverageConvergenceDivergenceExtended benchmark (`MACDEXT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 71.23M | 0.012 | 80.22M | 0.056 | 4.01× | 4.52× |
| 10,000 | 0.103 | 97.10M | 0.095 | 105.52M | 0.113 | 1.10× | 1.19× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.018 ms**; native kernel **0.017 ms**; TA-Lib 0.059 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.281 | 0.213 | 4.70M | 57.884 | 271.79× | 225.59× |
| 1,500 | 10 | 0.945 | 0.810 | 12.35M | 59.757 | 73.82× | 58.66× |
| 1,500 | 100 | 4.691 | 3.580 | 27.94M | 59.800 | 16.71× | 13.82× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 9.37M | 15.99M | 1.00× | 969.74K | 1.30M | 1.00× | 7.61M |
| 2 | 14.69M | 19.83M | 1.24× | 1.20M | 1.37M | 1.06× | 7.80M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
