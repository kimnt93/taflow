# MathExp benchmark (`EXP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.047 | 21.17M | 0.006 | 160.34M | 0.031 | 0.66× | 5.01× |
| 10,000 | 0.470 | 21.27M | 0.054 | 186.85M | 0.072 | 0.15× | 1.35× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
