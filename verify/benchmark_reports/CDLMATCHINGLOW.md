# CandleMatchingLow benchmark (`CDLMATCHINGLOW` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 114.83M | 0.007 | 142.22M | 0.032 | 3.69× | 4.57× |
| 10,000 | 0.047 | 212.35M | 0.044 | 226.99M | 0.089 | 1.89× | 2.02× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.010 ms**; native kernel **0.008 ms**; TA-Lib 0.037 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.418 | 0.264 | 3.78M | 34.200 | 129.35× | 106.72× |
| 1,500 | 10 | 2.482 | 1.191 | 8.40M | 35.775 | 30.04× | 24.41× |
| 1,500 | 100 | 5.272 | 2.866 | 34.89M | 35.524 | 12.39× | 9.96× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 10.13M | 15.43M | 1.00× | 975.71K | 977.85K | 1.00× | 9.45M |
| 2 | 16.87M | 16.52M | 1.07× | 1.19M | 1.34M | 1.37× | 10.25M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
