# AwesomeOscillator benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.029 | 34.01M | 0.029 | 34.77M | nan | — | — |
| 10,000 | 0.286 | 34.97M | 0.268 | 37.33M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
