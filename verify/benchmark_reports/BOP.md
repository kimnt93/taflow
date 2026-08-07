# BalanceOfPower benchmark (`BOP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.046 | 21.89M | 0.003 | 387.30M | 0.029 | 0.62× | 11.04× |
| 10,000 | 0.420 | 23.79M | 0.012 | 839.71M | 0.039 | 0.09× | 3.26× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
