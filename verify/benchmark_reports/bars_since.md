# BarsSince benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 225.47M | 0.004 | 276.01M | nan | — | — |
| 10,000 | 0.029 | 342.26M | 0.027 | 374.58M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
