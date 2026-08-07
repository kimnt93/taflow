# RollingMidpoint benchmark (`MIDPOINT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.065 | 15.43M | 0.026 | 38.98M | 0.036 | 0.56× | 1.42× |
| 10,000 | 0.644 | 15.52M | 0.248 | 40.40M | 0.097 | 0.15× | 0.39× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
