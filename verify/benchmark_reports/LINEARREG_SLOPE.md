# RollingLinearRegressionSlope benchmark (`LINEARREG_SLOPE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 80.39M | 0.011 | 88.02M | 0.039 | 3.15× | 3.44× |
| 10,000 | 0.109 | 91.55M | 0.105 | 95.65M | 0.131 | 1.20× | 1.26× |
| 100,000 | 1.044 | 95.74M | 1.036 | 96.54M | 1.037 | 0.99× | 1.00× |
| 1,000,000 | 10.725 | 93.24M | 10.184 | 98.20M | 10.001 | 0.93× | 0.98× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.043 ms**; native kernel **1.017 ms**; TA-Lib 1.021 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.234 | 0.172 | 5.81M | 1021.133 | 5933.72× | 164.64× |
| 100,000 | 10 | 1.098 | 0.732 | 13.67M | 1027.893 | 1404.70× | 38.53× |
| 100,000 | 1,000 | 17.023 | 12.085 | 82.75M | 1032.807 | 85.46× | 3.40× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 82.57M | 83.37M | 1.00× | 3.41M | 3.15M | 1.00× | 86.84M |
| 2 | 155.19M | 170.04M | 2.04× | 2.96M | 3.09M | 0.98× | 78.79M |
| 4 | 256.25M | 311.74M | 3.74× | 2.68M | 3.01M | 0.96× | 84.70M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
