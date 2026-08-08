# CandleMorningDojiStar benchmark (`CDLMORNINGDOJISTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 135.48M | 0.005 | 183.12M | 0.040 | 5.38× | 7.27× |
| 10,000 | 0.089 | 112.05M | 0.085 | 117.83M | 0.122 | 1.37× | 1.44× |
| 100,000 | 0.883 | 113.25M | 0.865 | 115.61M | 0.908 | 1.03× | 1.05× |
| 1,000,000 | 9.174 | 109.01M | 9.293 | 107.61M | 8.716 | 0.95× | 0.94× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.897 ms**; native kernel **0.866 ms**; TA-Lib 0.912 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.420 | 0.333 | 3.01M | 892.360 | 2682.11× | 100.81× |
| 100,000 | 10 | 2.806 | 1.531 | 6.53M | 918.293 | 599.85× | 21.28× |
| 100,000 | 1,000 | 36.175 | 31.359 | 31.89M | 895.235 | 28.55× | 1.24× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 102.30M | 103.72M | 1.00× | 2.42M | 2.44M | 1.00× | 97.87M |
| 2 | 186.45M | 185.10M | 1.78× | 2.29M | 2.41M | 0.99× | 95.65M |
| 4 | 342.72M | 365.67M | 3.53× | 2.24M | 2.60M | 1.07× | 97.83M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
