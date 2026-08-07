# CandleThreeStarsInSouth benchmark (`CDL3STARSINSOUTH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 80.78M | 0.011 | 92.65M | 0.035 | 2.82× | 3.24× |
| 10,000 | 0.122 | 82.27M | 0.118 | 84.58M | 0.114 | 0.94× | 0.97× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
