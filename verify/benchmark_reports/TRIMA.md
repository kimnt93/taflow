# TriangularMovingAverage benchmark (`TRIMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.050 | 19.94M | 0.009 | 113.87M | 0.035 | 0.69× | 3.95× |
| 10,000 | 0.478 | 20.92M | 0.081 | 123.91M | 0.062 | 0.13× | 0.77× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
