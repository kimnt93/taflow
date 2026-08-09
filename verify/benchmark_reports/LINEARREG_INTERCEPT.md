# RollingLinearRegressionIntercept benchmark (`LINEARREG_INTERCEPT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 65.49M | 0.014 | 70.26M | 0.044 | 2.88× | 3.09× |
| 10,000 | 0.140 | 71.50M | 0.134 | 74.42M | 0.151 | 1.08× | 1.12× |
| 100,000 | 1.354 | 73.83M | 1.320 | 75.76M | 1.194 | 0.88× | 0.90× |
| 1,000,000 | 13.710 | 72.94M | 13.276 | 75.32M | 11.938 | 0.87× | 0.90× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.347 ms**; native kernel **1.327 ms**; TA-Lib 1.216 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.242 | 0.163 | 6.13M | 1199.971 | 7355.70× | 179.69× |
| 100,000 | 10 | 1.053 | 0.823 | 12.16M | 1207.094 | 1467.33× | 37.03× |
| 100,000 | 1,000 | 15.699 | 17.503 | 57.13M | 1212.264 | 69.26× | 2.44× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 61.85M | 66.99M | 1.00× | 2.65M | 3.26M | 1.00× | 74.62M |
| 2 | 121.35M | 131.06M | 1.96× | 2.79M | 3.30M | 1.01× | 70.24M |
| 4 | 213.67M | 232.59M | 3.47× | 2.66M | 2.87M | 0.88× | 70.89M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
