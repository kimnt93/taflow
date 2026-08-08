# CandleHomingPigeon benchmark (`CDLHOMINGPIGEON` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 167.15M | 0.004 | 245.92M | 0.033 | 5.57× | 8.20× |
| 10,000 | 0.055 | 180.52M | 0.053 | 189.85M | 0.098 | 1.77× | 1.86× |
| 100,000 | 0.669 | 149.44M | 0.631 | 158.46M | 0.783 | 1.17× | 1.24× |
| 1,000,000 | 7.013 | 142.60M | 6.914 | 144.63M | 8.014 | 1.14× | 1.16× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.667 ms**; native kernel **0.658 ms**; TA-Lib 0.788 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.384 | 0.288 | 3.47M | 779.983 | 2710.15× | 99.57× |
| 100,000 | 10 | 2.789 | 1.456 | 6.87M | 799.068 | 548.93× | 18.77× |
| 100,000 | 1,000 | 28.050 | 25.270 | 39.57M | 814.266 | 32.22× | 1.27× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 119.55M | 121.76M | 1.00× | 2.25M | 2.52M | 1.00× | 110.00M |
| 2 | 240.44M | 238.41M | 1.96× | 2.32M | 2.63M | 1.04× | 108.40M |
| 4 | 363.29M | 375.45M | 3.08× | 2.29M | 2.54M | 1.01× | 109.27M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
