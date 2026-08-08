# RollingLinearRegressionSlope benchmark (`LINEARREG_SLOPE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.068 | 14.66M | 0.025 | 40.16M | 0.040 | 0.58× | 1.59× |
| 10,000 | 0.655 | 15.27M | 0.249 | 40.23M | 0.135 | 0.21× | 0.54× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.102 ms**; native kernel **0.037 ms**; TA-Lib 0.045 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.306 | 0.204 | 4.89M | 45.290 | 221.63× | 139.67× |
| 1,500 | 10 | 2.088 | 0.969 | 10.32M | 45.769 | 47.25× | 30.27× |
| 1,500 | 100 | 8.658 | 4.689 | 21.33M | 45.559 | 9.72× | 6.53× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
