# MinusDirectionalMovement benchmark (`MINUS_DM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 132.14M | 0.006 | 167.48M | 0.037 | 4.84× | 6.13× |
| 10,000 | 0.058 | 173.42M | 0.051 | 195.50M | 0.081 | 1.41× | 1.59× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
