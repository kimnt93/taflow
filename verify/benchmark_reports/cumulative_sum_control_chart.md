# CumulativeSumControlChart benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 168.83M | 0.005 | 207.22M | nan | — | — |
| 10,000 | 0.042 | 236.16M | 0.041 | 246.35M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
