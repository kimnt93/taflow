# CandleThreeWhiteSoldiers benchmark (`CDL3WHITESOLDIERS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.062 | 16.10M | 0.057 | 17.63M | 0.045 | 0.73× | 0.80× |
| 10,000 | 0.698 | 14.32M | 0.564 | 17.72M | 0.181 | 0.26× | 0.32× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
