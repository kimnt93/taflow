# StochasticOscillator benchmark (`STOCH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.203 | 4.92M | 0.054 | 18.59M | 0.057 | 0.28× | 1.05× |
| 10,000 | 2.131 | 4.69M | 0.544 | 18.37M | 0.167 | 0.08× | 0.31× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
