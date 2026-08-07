# AverageDirectionalIndexRating benchmark (`ADXR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.058 | 17.12M | 0.017 | 58.52M | 0.041 | 0.70× | 2.39× |
| 10,000 | 0.555 | 18.02M | 0.155 | 64.57M | 0.131 | 0.24× | 0.85× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
