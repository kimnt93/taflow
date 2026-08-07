# WeightedMovingAverage benchmark (`WMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.047 | 21.23M | 0.006 | 161.69M | 0.034 | 0.73× | 5.56× |
| 10,000 | 0.455 | 21.97M | 0.052 | 191.13M | 0.051 | 0.11× | 0.97× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
