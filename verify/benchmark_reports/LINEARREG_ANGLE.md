# RollingLinearRegressionAngle benchmark (`LINEARREG_ANGLE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.024 | 42.18M | 0.023 | 43.69M | 0.054 | 2.29× | 2.37× |
| 10,000 | 0.211 | 47.36M | 0.215 | 46.58M | 0.239 | 1.13× | 1.11× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.035 ms**; native kernel **0.035 ms**; TA-Lib 0.060 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.326 | 0.200 | 5.01M | 65.243 | 327.01× | 150.90× |
| 1,500 | 10 | 1.503 | 0.925 | 10.81M | 61.691 | 66.71× | 32.96× |
| 1,500 | 100 | 5.410 | 4.145 | 24.13M | 65.222 | 15.74× | 7.64× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 6.69M | 13.19M | 1.00× | 991.64K | 1.02M | 1.00× | 8.19M |
| 2 | 17.48M | 15.64M | 1.19× | 1.28M | 1.71M | 1.67× | 7.42M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
