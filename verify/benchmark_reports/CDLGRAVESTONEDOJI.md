# CandleGravestoneDoji benchmark (`CDLGRAVESTONEDOJI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 106.07M | 0.007 | 142.97M | 0.035 | 3.76× | 5.06× |
| 10,000 | 0.086 | 116.94M | 0.082 | 121.40M | 0.102 | 1.19× | 1.23× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
