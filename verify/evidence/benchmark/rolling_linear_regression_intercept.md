# RollingLinearRegressionIntercept benchmark (`LINEARREG_INTERCEPT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 65.48M | 0.015 | 67.40M | 0.048 | 3.15× | 3.25× |
| 10,000 | 0.122 | 82.06M | 0.142 | 70.29M | 0.160 | 1.31× | 1.12× |
| 100,000 | 1.618 | 61.81M | 1.201 | 83.26M | 1.261 | 0.78× | 1.05× |
| 1,000,000 | 12.326 | 81.13M | 12.137 | 82.39M | 11.767 | 0.95× | 0.97× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.106 | 0.105 | 0.99× |
| 1 | 5 | 0.314 | 0.455 | 1.45× |
| 1 | 10 | 0.478 | 0.971 | 2.03× |
| 10 | 1 | 0.054 | 0.091 | 1.70× |
| 10 | 5 | 0.229 | 0.644 | 2.81× |
| 10 | 10 | 0.586 | 1.144 | 1.95× |
| 100 | 1 | 0.067 | 0.092 | 1.38× |
| 100 | 5 | 0.278 | 0.523 | 1.88× |
| 100 | 10 | 0.776 | 1.149 | 1.48× |
| 1,000 | 1 | 0.075 | 0.110 | 1.47× |
| 1,000 | 5 | 0.303 | 0.713 | 2.35× |
| 1,000 | 10 | 0.627 | 1.218 | 1.94× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
