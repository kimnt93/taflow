# MathAtanh benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.048 | 20.72M | 0.005 | 183.38M | nan | — | — |
| 10,000 | 0.460 | 21.72M | 0.046 | 216.44M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
