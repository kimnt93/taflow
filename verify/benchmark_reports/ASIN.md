# MathAsin benchmark (`ASIN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.049 | 20.61M | 0.008 | 128.70M | 0.033 | 0.67× | 4.21× |
| 10,000 | 0.503 | 19.87M | 0.074 | 135.80M | 0.093 | 0.18× | 1.26× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
