# IntradayMomentumIndex benchmark (`IMI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.057 | 17.49M | 0.016 | 64.34M | 0.085 | 1.48× | 5.45× |
| 10,000 | 0.554 | 18.04M | 0.143 | 69.72M | 0.645 | 1.16× | 4.50× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
