# MathAcos benchmark (`ACOS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.050 | 20.15M | 0.008 | 119.72M | 0.034 | 0.68× | 4.05× |
| 10,000 | 0.483 | 20.72M | 0.076 | 132.06M | 0.096 | 0.20× | 1.27× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
