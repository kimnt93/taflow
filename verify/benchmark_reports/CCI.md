# CommodityChannelIndex benchmark (`CCI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.022 | 46.19M | 0.020 | 50.01M | 0.052 | 2.39× | 2.59× |
| 10,000 | 0.198 | 50.59M | 0.192 | 52.21M | 0.243 | 1.23× | 1.27× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
