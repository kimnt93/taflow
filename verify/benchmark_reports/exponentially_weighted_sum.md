# ExponentiallyWeightedSum benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 163.78M | 0.005 | 203.58M | nan | — | — |
| 10,000 | 0.041 | 245.69M | 0.039 | 256.65M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
