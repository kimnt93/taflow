# CandleLongLine benchmark (`CDLLONGLINE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 76.27M | 0.011 | 91.61M | 0.035 | 2.70× | 3.24× |
| 10,000 | 0.144 | 69.27M | 0.145 | 69.17M | 0.181 | 1.26× | 1.25× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
