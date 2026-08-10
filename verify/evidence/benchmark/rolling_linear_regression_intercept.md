# RollingLinearRegressionIntercept benchmark (`LINEARREG_INTERCEPT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 57.39M | 0.016 | 62.99M | 0.046 | 2.62× | 2.88× |
| 10,000 | 0.139 | 71.82M | 0.134 | 74.90M | 0.166 | 1.19× | 1.24× |
| 100,000 | 1.392 | 71.83M | 1.387 | 72.11M | 1.374 | 0.99× | 0.99× |
| 1,000,000 | 14.618 | 68.41M | 13.687 | 73.06M | 12.845 | 0.88× | 0.94× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.106 | 0.158 | 1.49× |
| 1 | 5 | 0.330 | 0.554 | 1.68× |
| 1 | 10 | 0.521 | 1.022 | 1.96× |
| 10 | 1 | 0.057 | 0.097 | 1.70× |
| 10 | 5 | 0.220 | 0.440 | 2.00× |
| 10 | 10 | 0.453 | 1.039 | 2.29× |
| 100 | 1 | 0.058 | 0.094 | 1.61× |
| 100 | 5 | 0.253 | 0.502 | 1.99× |
| 100 | 10 | 0.539 | 1.017 | 1.89× |
| 1,000 | 1 | 0.077 | 0.124 | 1.61× |
| 1,000 | 5 | 0.267 | 0.560 | 2.10× |
| 1,000 | 10 | 0.595 | 1.131 | 1.90× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
