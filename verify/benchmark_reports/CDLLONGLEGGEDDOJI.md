# CandleLongLeggedDoji benchmark (`CDLLONGLEGGEDDOJI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 112.42M | 0.007 | 142.29M | 0.035 | 3.94× | 4.99× |
| 10,000 | 0.082 | 121.50M | 0.079 | 125.91M | 0.092 | 1.12× | 1.16× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
