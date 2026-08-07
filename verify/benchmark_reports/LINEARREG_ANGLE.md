# RollingLinearRegressionAngle benchmark (`LINEARREG_ANGLE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.077 | 12.98M | 0.037 | 27.37M | 0.052 | 0.67× | 1.41× |
| 10,000 | 0.781 | 12.80M | 0.369 | 27.13M | 0.247 | 0.32× | 0.67× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
