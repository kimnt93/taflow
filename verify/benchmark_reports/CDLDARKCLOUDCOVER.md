# CandleDarkCloudCover benchmark (`CDLDARKCLOUDCOVER` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 182.18M | 0.004 | 277.48M | 0.035 | 6.32× | 9.63× |
| 10,000 | 0.073 | 136.08M | 0.068 | 146.79M | 0.112 | 1.53× | 1.65× |
| 100,000 | 0.847 | 118.04M | 0.835 | 119.77M | 0.836 | 0.99× | 1.00× |
| 1,000,000 | 8.964 | 111.55M | 8.884 | 112.56M | 8.376 | 0.93× | 0.94× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.847 ms**; native kernel **0.829 ms**; TA-Lib 0.851 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.381 | 0.340 | 2.94M | 836.352 | 2460.33× | 94.73× |
| 100,000 | 10 | 2.835 | 1.407 | 7.11M | 857.919 | 609.76× | 22.58× |
| 100,000 | 1,000 | 27.892 | 28.045 | 35.66M | 842.752 | 30.05× | 1.23× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 95.24M | 97.06M | 1.00× | 2.13M | 2.10M | 1.00× | 85.81M |
| 2 | 195.53M | 199.70M | 2.06× | 2.07M | 2.45M | 1.17× | 100.65M |
| 4 | 313.77M | 356.52M | 3.67× | 2.19M | 2.60M | 1.24× | 99.94M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
