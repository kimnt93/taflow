# AverageTrueRange benchmark (`ATR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.048 | 20.98M | 0.007 | 153.80M | 0.039 | 0.81× | 5.97× |
| 10,000 | 0.464 | 21.54M | 0.055 | 183.04M | 0.089 | 0.19× | 1.62× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
