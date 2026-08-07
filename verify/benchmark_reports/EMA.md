# ExponentialMovingAverage benchmark (`EMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 177.21M | 0.005 | 216.99M | 0.034 | 6.06× | 7.43× |
| 10,000 | 0.041 | 243.67M | 0.038 | 265.85M | 0.059 | 1.44× | 1.57× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
