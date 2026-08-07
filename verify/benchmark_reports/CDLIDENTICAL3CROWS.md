# CandleIdenticalThreeCrows benchmark (`CDLIDENTICAL3CROWS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.042 | 23.89M | 0.040 | 25.00M | 0.036 | 0.85× | 0.89× |
| 10,000 | 0.404 | 24.78M | 0.404 | 24.77M | 0.116 | 0.29× | 0.29× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
