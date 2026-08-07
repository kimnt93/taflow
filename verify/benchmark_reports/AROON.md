# Aroon benchmark (`AROON` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.129 | 7.78M | 0.042 | 23.96M | 0.042 | 0.32× | 1.00× |
| 10,000 | 1.263 | 7.92M | 0.458 | 21.82M | 0.151 | 0.12× | 0.33× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
