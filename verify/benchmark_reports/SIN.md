# MathSin benchmark (`SIN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.052 | 19.08M | 0.011 | 89.92M | 0.037 | 0.70× | 3.31× |
| 10,000 | 0.561 | 17.82M | 0.150 | 66.61M | 0.177 | 0.32× | 1.18× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
