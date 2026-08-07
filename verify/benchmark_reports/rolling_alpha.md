# RollingAlpha benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.060 | 16.64M | 0.057 | 17.52M | nan | — | — |
| 10,000 | 0.576 | 17.36M | 0.569 | 17.56M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
