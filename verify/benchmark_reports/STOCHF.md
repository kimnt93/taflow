# FastStochasticOscillator benchmark (`STOCHF` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.204 | 4.91M | 0.049 | 20.43M | 0.046 | 0.23× | 0.95× |
| 10,000 | 2.056 | 4.86M | 0.520 | 19.21M | 0.144 | 0.07× | 0.28× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
