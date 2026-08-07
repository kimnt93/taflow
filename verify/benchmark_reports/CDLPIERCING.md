# CandlePiercing benchmark (`CDLPIERCING` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.028 | 35.46M | 0.026 | 38.80M | 0.032 | 1.13× | 1.23× |
| 10,000 | 0.270 | 37.05M | 0.267 | 37.41M | 0.122 | 0.45× | 0.46× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
