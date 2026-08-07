# MathFloor benchmark (`FLOOR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.045 | 22.19M | 0.004 | 276.23M | 0.028 | 0.62× | 7.76× |
| 10,000 | 0.432 | 23.14M | 0.027 | 370.35M | 0.040 | 0.09× | 1.48× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
