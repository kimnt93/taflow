# MathAtan benchmark (`ATAN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.049 | 20.24M | 0.008 | 129.54M | 0.032 | 0.64× | 4.09× |
| 10,000 | 0.475 | 21.05M | 0.069 | 145.84M | 0.087 | 0.18× | 1.27× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
