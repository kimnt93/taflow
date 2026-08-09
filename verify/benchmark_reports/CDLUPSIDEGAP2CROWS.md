# CandleUpsideGapTwoCrows benchmark (`CDLUPSIDEGAP2CROWS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 112.18M | 0.007 | 135.59M | 0.032 | 3.55× | 4.29× |
| 10,000 | 0.085 | 117.33M | 0.083 | 120.78M | 0.116 | 1.36× | 1.40× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.010 ms**; native kernel **0.008 ms**; TA-Lib 0.034 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.342 | 0.269 | 3.71M | 34.414 | 127.79× | 109.56× |
| 1,500 | 10 | 2.545 | 1.255 | 7.97M | 35.050 | 27.93× | 22.61× |
| 1,500 | 100 | 5.472 | 3.327 | 30.05M | 34.454 | 10.35× | 8.77× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 6.86M | 8.86M | 1.00× | 1.15M | 1.33M | 1.00× | 8.90M |
| 2 | 15.80M | 15.75M | 1.78× | 1.28M | 1.31M | 0.99× | 8.80M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
