# CandleDragonflyDoji benchmark (`CDLDRAGONFLYDOJI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 102.65M | 0.007 | 136.12M | 0.034 | 3.54× | 4.70× |
| 10,000 | 0.082 | 121.96M | 0.080 | 125.12M | 0.098 | 1.19× | 1.22× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
