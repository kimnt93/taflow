# MoneyFlowIndex benchmark (`MFI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 69.21M | 0.013 | 79.38M | 0.036 | 2.53× | 2.90× |
| 10,000 | 0.153 | 65.28M | 0.145 | 68.88M | 0.112 | 0.73× | 0.77× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
