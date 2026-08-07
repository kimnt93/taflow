# FracDiff benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.080 | 12.50M | 0.076 | 13.15M | nan | — | — |
| 10,000 | 8.128 | 1.23M | 8.043 | 1.24M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
