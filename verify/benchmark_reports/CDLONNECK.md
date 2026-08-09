# CandleOnNeck benchmark (`CDLONNECK` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 167.78M | 0.004 | 243.20M | 0.032 | 5.45× | 7.90× |
| 10,000 | 0.061 | 164.87M | 0.056 | 179.08M | 0.118 | 1.95× | 2.11× |
| 100,000 | 0.892 | 112.12M | 0.877 | 114.00M | 0.920 | 1.03× | 1.05× |
| 1,000,000 | 9.426 | 106.09M | 9.556 | 104.65M | 9.134 | 0.97× | 0.96× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.907 ms**; native kernel **0.882 ms**; TA-Lib 0.918 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.338 | 0.269 | 3.71M | 915.340 | 3399.71× | 103.92× |
| 100,000 | 10 | 2.673 | 1.363 | 7.34M | 924.957 | 678.77× | 20.73× |
| 100,000 | 1,000 | 29.556 | 26.243 | 38.11M | 932.020 | 35.52× | 1.25× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 90.73M | 95.74M | 1.00× | 2.37M | 2.44M | 1.00× | 90.51M |
| 2 | 180.18M | 189.83M | 1.98× | 2.41M | 2.60M | 1.06× | 91.10M |
| 4 | 311.46M | 345.98M | 3.61× | 2.21M | 2.45M | 1.00× | 91.51M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
