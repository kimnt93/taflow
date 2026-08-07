# RollingTimeSeriesForecast benchmark (`TSF` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.069 | 14.49M | 0.028 | 35.24M | 0.047 | 0.69× | 1.67× |
| 10,000 | 0.696 | 14.37M | 0.272 | 36.82M | 0.169 | 0.24× | 0.62× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
