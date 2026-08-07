# CandleSeparatingLines benchmark (`CDLSEPARATINGLINES` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.029 | 34.63M | 0.028 | 36.03M | 0.035 | 1.20× | 1.25× |
| 10,000 | 0.265 | 37.71M | 0.261 | 38.35M | 0.128 | 0.48× | 0.49× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
