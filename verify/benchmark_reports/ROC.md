# RateOfChange benchmark (`ROC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.046 | 21.54M | 0.005 | 221.74M | 0.032 | 0.68× | 6.99× |
| 10,000 | 0.438 | 22.81M | 0.036 | 275.80M | 0.041 | 0.09× | 1.14× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
