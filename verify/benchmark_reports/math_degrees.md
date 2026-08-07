# MathDegrees benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.044 | 22.64M | 0.002 | 496.34M | nan | — | — |
| 10,000 | 0.436 | 22.91M | 0.014 | 699.67M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
