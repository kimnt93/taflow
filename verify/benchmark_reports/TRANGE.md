# TrueRange benchmark (`TRANGE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.044 | 22.69M | 0.002 | 436.50M | 0.027 | 0.61× | 11.74× |
| 10,000 | 0.429 | 23.29M | 0.013 | 780.47M | 0.034 | 0.08× | 2.65× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
