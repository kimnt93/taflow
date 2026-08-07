# CandleMorningStar benchmark (`CDLMORNINGSTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.039 | 25.59M | 0.038 | 26.21M | 0.038 | 0.96× | 0.98× |
| 10,000 | 0.411 | 24.31M | 0.400 | 25.00M | 0.113 | 0.28× | 0.28× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
