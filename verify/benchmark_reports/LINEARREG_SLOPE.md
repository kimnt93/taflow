# RollingLinearRegressionSlope benchmark (`LINEARREG_SLOPE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.067 | 14.83M | 0.026 | 37.95M | 0.042 | 0.62× | 1.58× |
| 10,000 | 0.674 | 14.84M | 0.252 | 39.61M | 0.138 | 0.20× | 0.55× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
