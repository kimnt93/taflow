# RollingZScore benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 47.51M | 0.020 | 50.89M | nan | — | — |
| 10,000 | 0.188 | 53.14M | 0.187 | 53.42M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
