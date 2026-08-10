# MinusDirectionalMovement benchmark (`MINUS_DM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 109.64M | 0.008 | 120.23M | 0.036 | 3.89× | 4.27× |
| 10,000 | 0.055 | 183.31M | 0.053 | 188.67M | 0.091 | 1.68× | 1.72× |
| 100,000 | 0.509 | 196.57M | 0.500 | 200.08M | 0.513 | 1.01× | 1.03× |
| 1,000,000 | 5.455 | 183.32M | 5.106 | 195.83M | 4.812 | 0.88× | 0.94× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.113 | 0.196 | 1.73× |
| 1 | 5 | 0.288 | 0.478 | 1.66× |
| 1 | 10 | 0.506 | 0.954 | 1.88× |
| 10 | 1 | 0.045 | 0.099 | 2.18× |
| 10 | 5 | 0.239 | 0.463 | 1.94× |
| 10 | 10 | 0.472 | 0.918 | 1.95× |
| 100 | 1 | 0.048 | 0.088 | 1.83× |
| 100 | 5 | 0.225 | 0.457 | 2.03× |
| 100 | 10 | 0.459 | 0.930 | 2.02× |
| 1,000 | 1 | 0.053 | 0.096 | 1.81× |
| 1,000 | 5 | 0.231 | 0.451 | 1.95× |
| 1,000 | 10 | 0.481 | 1.004 | 2.09× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
