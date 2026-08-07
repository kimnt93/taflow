# TripleExponentialMovingAverage benchmark (`TEMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.053 | 18.71M | 0.013 | 78.65M | 0.040 | 0.75× | 3.17× |
| 10,000 | 0.515 | 19.41M | 0.118 | 84.52M | 0.120 | 0.23× | 1.01× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
