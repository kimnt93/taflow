# AbsolutePriceOscillator benchmark (`APO` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.053 | 18.93M | 0.011 | 88.07M | 0.038 | 0.73× | 3.37× |
| 10,000 | 0.512 | 19.51M | 0.101 | 99.44M | 0.075 | 0.15× | 0.74× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
