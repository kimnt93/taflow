# RollingSkew benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.038 | 26.27M | 0.040 | 24.97M | nan | — | — |
| 10,000 | 0.351 | 28.51M | 0.383 | 26.09M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
