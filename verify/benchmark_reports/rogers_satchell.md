# RogersSatchell benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.033 | 30.69M | 0.030 | 32.82M | nan | — | — |
| 10,000 | 0.292 | 34.30M | 0.290 | 34.52M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
