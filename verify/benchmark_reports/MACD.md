# MovingAverageConvergenceDivergence benchmark (`MACD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.212 | 4.72M | 0.016 | 60.61M | 0.052 | 0.25× | 3.15× |
| 10,000 | 2.117 | 4.72M | 0.148 | 67.76M | 0.139 | 0.07× | 0.94× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
