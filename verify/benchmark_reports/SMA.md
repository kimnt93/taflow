# SimpleMovingAverage benchmark (`SMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.046 | 21.79M | 0.005 | 188.42M | 0.033 | 0.73× | 6.30× |
| 10,000 | 0.477 | 20.96M | 0.045 | 220.80M | 0.052 | 0.11× | 1.14× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
