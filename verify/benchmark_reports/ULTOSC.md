# UltimateOscillator benchmark (`ULTOSC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.039 | 25.44M | 0.038 | 26.58M | 0.052 | 1.33× | 1.39× |
| 10,000 | 0.357 | 28.01M | 0.364 | 27.49M | 0.185 | 0.52× | 0.51× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
