# RollingStandardDeviation benchmark (`STDDEV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.052 | 19.34M | 0.009 | 115.07M | 0.036 | 0.70× | 4.18× |
| 10,000 | 0.493 | 20.29M | 0.082 | 122.13M | 0.062 | 0.13× | 0.76× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
