# TypicalPrice benchmark (`TYPPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.055 | 18.19M | 0.002 | 649.80M | 0.030 | 0.55× | 19.74× |
| 10,000 | 0.418 | 23.94M | 0.006 | 1.65G | 0.035 | 0.08× | 5.81× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
