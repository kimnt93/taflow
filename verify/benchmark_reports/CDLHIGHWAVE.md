# CandleHighWave benchmark (`CDLHIGHWAVE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 90.94M | 0.009 | 114.28M | 0.034 | 3.13× | 3.94× |
| 10,000 | 0.139 | 71.77M | 0.136 | 73.29M | 0.159 | 1.14× | 1.17× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
