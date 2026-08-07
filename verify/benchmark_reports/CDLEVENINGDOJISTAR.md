# CandleEveningDojiStar benchmark (`CDLEVENINGDOJISTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.038 | 26.16M | 0.036 | 27.44M | 0.039 | 1.02× | 1.07× |
| 10,000 | 0.382 | 26.18M | 0.374 | 26.71M | 0.120 | 0.31× | 0.32× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
