# PremiumDiscount benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.041 | 24.43M | 0.040 | 25.21M | nan | — | — |
| 10,000 | 0.383 | 26.13M | 0.377 | 26.51M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
