# RollingLinearRegressionSlope benchmark (`LINEARREG_SLOPE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 69.66M | 0.016 | 63.06M | 0.041 | 2.87× | 2.60× |
| 10,000 | 0.124 | 80.60M | 0.112 | 89.46M | 0.147 | 1.18× | 1.31× |
| 100,000 | 1.327 | 75.37M | 1.119 | 89.36M | 1.073 | 0.81× | 0.96× |
| 1,000,000 | 11.714 | 85.37M | 10.987 | 91.01M | 10.649 | 0.91× | 0.97× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.173 ms**; native kernel **1.136 ms**; TA-Lib 1.146 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.250 | 0.186 | 5.39M | 1146.178 | 6175.92× | 172.18× |
| 100,000 | 10 | 1.185 | 0.766 | 13.05M | 1154.177 | 1506.19× | 42.19× |
| 100,000 | 1,000 | 15.056 | 13.611 | 73.47M | 1113.255 | 81.79× | 3.10× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 81.94M | 85.86M | 1.00× | 3.54M | 3.05M | 1.00× | 85.17M |
| 2 | 149.23M | 149.22M | 1.74× | 2.95M | 3.57M | 1.17× | 84.54M |
| 4 | 265.37M | 319.74M | 3.72× | 2.82M | 2.75M | 0.90× | 79.16M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
