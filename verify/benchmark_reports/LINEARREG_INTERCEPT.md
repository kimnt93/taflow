# RollingLinearRegressionIntercept benchmark (`LINEARREG_INTERCEPT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 66.45M | 0.014 | 71.45M | 0.047 | 3.15× | 3.38× |
| 10,000 | 0.132 | 75.90M | 0.128 | 77.86M | 0.154 | 1.17× | 1.20× |
| 100,000 | 1.300 | 76.90M | 1.263 | 79.21M | 1.612 | 1.24× | 1.28× |
| 1,000,000 | 13.873 | 72.08M | 17.110 | 58.45M | 13.978 | 1.01× | 0.82× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.296 ms**; native kernel **1.250 ms**; TA-Lib 1.274 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.249 | 0.171 | 5.85M | 1270.346 | 7431.15× | 172.88× |
| 100,000 | 10 | 1.097 | 0.821 | 12.17M | 1233.518 | 1501.69× | 38.07× |
| 100,000 | 1,000 | 17.280 | 14.478 | 69.07M | 1307.443 | 90.31× | 3.88× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 65.18M | 65.09M | 1.00× | 2.74M | 3.11M | 1.00× | 67.02M |
| 2 | 95.76M | 136.60M | 2.10× | 2.53M | 3.08M | 0.99× | 65.79M |
| 4 | 161.72M | 204.94M | 3.15× | 2.52M | 2.69M | 0.86× | 65.95M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
