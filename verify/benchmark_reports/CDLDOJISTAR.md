# CandleDojiStar benchmark (`CDLDOJISTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 92.38M | 0.009 | 114.03M | 0.037 | 3.44× | 4.25× |
| 10,000 | 0.110 | 90.62M | 0.107 | 93.66M | 0.141 | 1.28× | 1.32× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.012 ms**; native kernel **0.010 ms**; TA-Lib 0.043 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.414 | 0.304 | 3.29M | 42.234 | 139.15× | 98.50× |
| 1,500 | 10 | 2.680 | 1.341 | 7.45M | 42.085 | 31.37× | 21.97× |
| 1,500 | 100 | 6.044 | 12.073 | 8.28M | 45.107 | 3.74× | 2.60× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.42M | 10.79M | 1.00× | 1.12M | 886.74K | 1.00× | 8.83M |
| 2 | 17.29M | 17.56M | 1.63× | 1.28M | 1.41M | 1.60× | 8.77M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
