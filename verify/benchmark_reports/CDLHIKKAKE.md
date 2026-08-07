# CandleHikkake benchmark (`CDLHIKKAKE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 102.04M | 0.008 | 127.60M | 0.030 | 3.08× | 3.85× |
| 10,000 | 0.110 | 91.19M | 0.103 | 97.49M | 0.076 | 0.70× | 0.74× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
