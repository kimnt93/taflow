# MathLn benchmark (`LN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.047 | 21.37M | 0.006 | 176.52M | 0.031 | 0.67× | 5.50× |
| 10,000 | 0.452 | 22.11M | 0.048 | 208.12M | 0.068 | 0.15× | 1.42× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
