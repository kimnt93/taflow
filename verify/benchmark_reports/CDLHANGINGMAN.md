# CandleHangingMan benchmark (`CDLHANGINGMAN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.032 | 30.90M | 0.030 | 33.85M | 0.040 | 1.25× | 1.36× |
| 10,000 | 0.303 | 33.05M | 0.298 | 33.55M | 0.176 | 0.58× | 0.59× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
