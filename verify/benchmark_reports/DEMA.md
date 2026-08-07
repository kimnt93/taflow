# DoubleExponentialMovingAverage benchmark (`DEMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.051 | 19.44M | 0.009 | 108.39M | 0.039 | 0.75× | 4.18× |
| 10,000 | 0.473 | 21.14M | 0.082 | 121.72M | 0.093 | 0.20× | 1.13× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
