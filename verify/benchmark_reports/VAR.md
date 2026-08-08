# RollingVariance benchmark (`VAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 180.45M | 0.004 | 226.57M | 0.035 | 6.34× | 7.97× |
| 10,000 | 0.040 | 251.49M | 0.034 | 292.78M | 0.053 | 1.34× | 1.56× |
| 100,000 | 0.355 | 281.62M | 0.329 | 304.19M | 0.238 | 0.67× | 0.72× |
| 1,000,000 | 4.037 | 247.69M | 3.455 | 289.46M | 2.225 | 0.55× | 0.64× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.362 ms**; native kernel **0.338 ms**; TA-Lib 0.238 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.236 | 0.146 | 6.87M | 246.770 | 1694.60× | 215.60× |
| 100,000 | 10 | 0.926 | 0.542 | 18.44M | 238.215 | 439.21× | 58.95× |
| 100,000 | 1,000 | 6.010 | 4.777 | 209.35M | 236.330 | 49.48× | 7.40× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 156.61M | 205.26M | 1.00× | 3.15M | 3.03M | 1.00× | 272.56M |
| 2 | 303.49M | 471.64M | 2.30× | 2.79M | 4.13M | 1.36× | 296.38M |
| 4 | 475.52M | 754.47M | 3.68× | 3.27M | 3.46M | 1.14× | 307.23M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
