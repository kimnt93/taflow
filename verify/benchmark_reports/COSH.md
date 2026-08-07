# MathCosh benchmark (`COSH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.049 | 20.24M | 0.008 | 127.63M | 0.033 | 0.67× | 4.25× |
| 10,000 | 0.476 | 21.03M | 0.067 | 149.35M | 0.087 | 0.18× | 1.30× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
