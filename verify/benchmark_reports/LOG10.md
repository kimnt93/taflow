# MathLog10 benchmark (`LOG10` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.052 | 19.40M | 0.009 | 107.45M | 0.035 | 0.68× | 3.77× |
| 10,000 | 0.488 | 20.47M | 0.084 | 118.58M | 0.105 | 0.22× | 1.25× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
