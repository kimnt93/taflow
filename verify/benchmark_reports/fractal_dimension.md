# FractalDimension benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.125 | 8.00M | 0.121 | 8.24M | nan | — | — |
| 10,000 | 1.231 | 8.12M | 1.228 | 8.15M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
