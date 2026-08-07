# TripleExponentialRateOfChange benchmark (`TRIX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 69.02M | 0.013 | 76.88M | 0.039 | 2.67× | 2.97× |
| 10,000 | 0.131 | 76.33M | 0.125 | 80.12M | 0.121 | 0.92× | 0.97× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
