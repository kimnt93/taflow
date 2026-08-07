# RollingMode benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.029 | 34.84M | 0.028 | 36.10M | nan | — | — |
| 10,000 | 0.273 | 36.69M | 0.265 | 37.77M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
