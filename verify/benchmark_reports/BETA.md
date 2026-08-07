# RollingBeta benchmark (`BETA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.270 | 3.71M | 0.009 | 106.37M | 0.039 | 0.14× | 4.12× |
| 10,000 | 2.647 | 3.78M | 0.082 | 122.27M | 0.088 | 0.03× | 1.07× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
