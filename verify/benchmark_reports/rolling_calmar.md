# RollingCalmar benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.028 | 35.48M | 0.027 | 37.58M | nan | — | — |
| 10,000 | 0.250 | 39.95M | 0.248 | 40.31M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
