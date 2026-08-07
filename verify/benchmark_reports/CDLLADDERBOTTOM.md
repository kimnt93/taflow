# CandleLadderBottom benchmark (`CDLLADDERBOTTOM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.022 | 45.57M | 0.020 | 50.26M | 0.032 | 1.47× | 1.63× |
| 10,000 | 0.201 | 49.76M | 0.203 | 49.35M | 0.086 | 0.43× | 0.42× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
