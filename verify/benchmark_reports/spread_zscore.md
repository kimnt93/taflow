# SpreadZScore benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.108 | 9.27M | 0.107 | 9.33M | nan | — | — |
| 10,000 | 1.071 | 9.33M | 1.036 | 9.65M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
