# RollingLinearRegressionAngle benchmark (`LINEARREG_ANGLE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.022 | 44.99M | 0.021 | 46.73M | 0.049 | 2.22× | 2.30× |
| 10,000 | 0.210 | 47.73M | 0.204 | 49.01M | 0.231 | 1.10× | 1.13× |
| 100,000 | 2.115 | 47.27M | 2.056 | 48.63M | 2.070 | 0.98× | 1.01× |
| 1,000,000 | 21.078 | 47.44M | 20.931 | 47.78M | 20.606 | 0.98× | 0.98× |

## Warm-up

Construct + canonical extend over 100,000 bars: **2.136 ms**; native kernel **2.055 ms**; TA-Lib 2.058 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.269 | 0.171 | 5.86M | 2071.156 | 12139.01× | 167.74× |
| 100,000 | 10 | 1.327 | 0.938 | 10.66M | 2235.118 | 2383.25× | 30.39× |
| 100,000 | 1,000 | 23.600 | 22.535 | 44.38M | 2106.517 | 93.48× | 2.17× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 40.96M | 43.68M | 1.00× | 2.62M | 2.82M | 1.00× | 42.94M |
| 2 | 78.45M | 78.23M | 1.79× | 2.43M | 2.96M | 1.05× | 43.16M |
| 4 | 140.67M | 166.22M | 3.81× | 2.42M | 2.62M | 0.93× | 42.26M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
