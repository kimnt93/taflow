# RollingLinearRegression benchmark (`LINEARREG` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.069 | 14.54M | 0.027 | 36.93M | 0.045 | 0.66× | 1.66× |
| 10,000 | 0.660 | 15.15M | 0.264 | 37.83M | 0.162 | 0.25× | 0.61× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
