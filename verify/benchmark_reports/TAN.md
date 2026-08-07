# MathTan benchmark (`TAN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.060 | 16.78M | 0.018 | 56.25M | 0.043 | 0.73× | 2.43× |
| 10,000 | 0.610 | 16.40M | 0.203 | 49.23M | 0.223 | 0.37× | 1.10× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
