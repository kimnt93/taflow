# ChaikinMoneyFlow benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 74.50M | 0.011 | 88.56M | nan | — | — |
| 10,000 | 0.102 | 98.45M | 0.097 | 102.84M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
