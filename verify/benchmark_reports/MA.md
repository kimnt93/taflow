# MovingAverage benchmark (`MA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.048 | 20.99M | 0.007 | 145.30M | 0.036 | 0.75× | 5.19× |
| 10,000 | 0.452 | 22.11M | 0.059 | 170.43M | 0.055 | 0.12× | 0.93× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
