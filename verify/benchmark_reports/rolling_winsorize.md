# RollingWinsorize benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.059 | 16.82M | 0.059 | 17.03M | nan | — | — |
| 10,000 | 0.597 | 16.74M | 0.739 | 13.53M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
