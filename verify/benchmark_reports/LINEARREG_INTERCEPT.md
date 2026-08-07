# RollingLinearRegressionIntercept benchmark (`LINEARREG_INTERCEPT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.068 | 14.71M | 0.027 | 37.67M | 0.043 | 0.64× | 1.63× |
| 10,000 | 0.657 | 15.23M | 0.261 | 38.35M | 0.158 | 0.24× | 0.61× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
