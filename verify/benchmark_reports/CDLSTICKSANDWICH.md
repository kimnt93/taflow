# CandleStickSandwich benchmark (`CDLSTICKSANDWICH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 86.43M | 0.009 | 111.04M | 0.032 | 2.81× | 3.60× |
| 10,000 | 0.120 | 83.01M | 0.116 | 86.23M | 0.088 | 0.73× | 0.76× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
