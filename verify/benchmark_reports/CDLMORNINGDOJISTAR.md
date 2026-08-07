# CandleMorningDojiStar benchmark (`CDLMORNINGDOJISTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.039 | 25.96M | 0.036 | 27.79M | 0.039 | 1.02× | 1.10× |
| 10,000 | 0.383 | 26.14M | 0.388 | 25.76M | 0.123 | 0.32× | 0.32× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
