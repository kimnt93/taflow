# CandleEngulfing benchmark (`CDLENGULFING` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 145.94M | 0.005 | 204.81M | 0.032 | 4.63× | 6.50× |
| 10,000 | 0.075 | 132.95M | 0.073 | 136.09M | 0.083 | 1.11× | 1.13× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
