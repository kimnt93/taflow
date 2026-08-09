# CandleMarubozu benchmark (`CDLMARUBOZU` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 146.37M | 0.005 | 215.37M | 0.034 | 5.03× | 7.40× |
| 10,000 | 0.092 | 108.71M | 0.090 | 110.79M | 0.128 | 1.39× | 1.42× |
| 100,000 | 1.010 | 99.00M | 0.967 | 103.44M | 1.022 | 1.01× | 1.06× |
| 1,000,000 | 10.249 | 97.57M | 10.260 | 97.47M | 10.478 | 1.02× | 1.02× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.980 ms**; native kernel **0.956 ms**; TA-Lib 1.034 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.345 | 0.263 | 3.80M | 1029.969 | 3909.21× | 102.05× |
| 100,000 | 10 | 2.542 | 1.323 | 7.56M | 1004.362 | 759.02× | 20.32× |
| 100,000 | 1,000 | 28.355 | 26.048 | 38.39M | 1058.078 | 40.62× | 1.27× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 86.57M | 87.71M | 1.00× | 2.25M | 2.65M | 1.00× | 86.30M |
| 2 | 175.41M | 174.72M | 1.99× | 2.47M | 2.71M | 1.02× | 85.17M |
| 4 | 300.04M | 314.89M | 3.59× | 2.33M | 2.53M | 0.96× | 81.47M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
