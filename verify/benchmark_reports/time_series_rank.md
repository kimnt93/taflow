# TimeSeriesRank benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 59.90M | 0.015 | 65.02M | nan | — | — |
| 10,000 | 0.144 | 69.67M | 0.140 | 71.50M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
