# HedgeRatio benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.061 | 16.48M | 0.056 | 17.77M | nan | — | — |
| 10,000 | 0.581 | 17.22M | 0.564 | 17.72M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
