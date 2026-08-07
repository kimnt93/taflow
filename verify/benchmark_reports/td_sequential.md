# TomDeMarkSequential benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 142.28M | 0.006 | 160.06M | nan | — | — |
| 10,000 | 0.062 | 160.03M | 0.059 | 168.35M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
