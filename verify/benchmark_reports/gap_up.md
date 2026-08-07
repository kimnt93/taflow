# GapUp benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 181.23M | 0.004 | 245.68M | nan | — | — |
| 10,000 | 0.034 | 296.62M | 0.031 | 323.07M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
