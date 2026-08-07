# TrueStrengthIndex benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 119.23M | 0.007 | 141.55M | nan | — | — |
| 10,000 | 0.062 | 160.07M | 0.059 | 168.90M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
