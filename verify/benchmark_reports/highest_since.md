# HighestSince benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 165.20M | 0.005 | 208.39M | nan | — | — |
| 10,000 | 0.041 | 243.28M | 0.039 | 258.09M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
