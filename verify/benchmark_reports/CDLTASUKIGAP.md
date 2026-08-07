# CandleTasukiGap benchmark (`CDLTASUKIGAP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 54.74M | 0.016 | 63.37M | 0.043 | 2.37× | 2.74× |
| 10,000 | 0.196 | 50.95M | 0.192 | 52.00M | 0.177 | 0.90× | 0.92× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
