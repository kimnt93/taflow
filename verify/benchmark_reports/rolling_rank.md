# RollingRank benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 61.67M | 0.015 | 66.29M | nan | — | — |
| 10,000 | 0.146 | 68.55M | 0.144 | 69.32M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
