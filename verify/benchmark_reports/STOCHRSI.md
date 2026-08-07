# StochasticRelativeStrengthIndex benchmark (`STOCHRSI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.203 | 4.92M | 0.059 | 16.98M | 0.056 | 0.28× | 0.95× |
| 10,000 | 2.155 | 4.64M | 0.618 | 16.18M | 0.208 | 0.10× | 0.34× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
