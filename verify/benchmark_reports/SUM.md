# RollingSum benchmark (`SUM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 209.42M | 0.004 | 261.83M | 0.031 | 6.53× | 8.16× |
| 10,000 | 0.034 | 295.58M | 0.030 | 332.04M | 0.050 | 1.49× | 1.67× |
| 100,000 | 0.322 | 310.20M | 0.290 | 344.23M | 0.205 | 0.64× | 0.71× |
| 1,000,000 | 3.428 | 291.70M | 2.995 | 333.86M | 1.851 | 0.54× | 0.62× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.318 ms**; native kernel **0.295 ms**; TA-Lib 0.206 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.213 | 0.192 | 5.20M | 211.494 | 1098.96× | 147.22× |
| 100,000 | 10 | 0.839 | 0.530 | 18.87M | 206.673 | 389.97× | 53.03× |
| 100,000 | 1,000 | 6.138 | 4.321 | 231.43M | 206.536 | 47.80× | 7.21× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 201.49M | 280.68M | 1.00× | 3.53M | 3.04M | 1.00× | 360.03M |
| 2 | 385.50M | 434.71M | 1.55× | 3.22M | 3.47M | 1.14× | 341.40M |
| 4 | 566.22M | 890.42M | 3.17× | 3.25M | 2.88M | 0.95× | 328.24M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
