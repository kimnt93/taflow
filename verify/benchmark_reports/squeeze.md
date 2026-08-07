# Squeeze benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.043 | 23.51M | 0.039 | 25.33M | nan | — | — |
| 10,000 | 0.373 | 26.84M | 0.364 | 27.48M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
