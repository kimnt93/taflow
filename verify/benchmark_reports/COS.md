# MathCos benchmark (`COS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.052 | 19.19M | 0.011 | 88.38M | 0.038 | 0.74× | 3.40× |
| 10,000 | 0.552 | 18.12M | 0.151 | 66.35M | 0.179 | 0.32× | 1.19× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
