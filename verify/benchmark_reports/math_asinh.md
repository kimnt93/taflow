# MathAsinh benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.054 | 18.48M | 0.013 | 79.12M | nan | — | — |
| 10,000 | 0.525 | 19.05M | 0.120 | 83.23M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
