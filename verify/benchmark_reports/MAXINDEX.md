# RollingArgmax benchmark (`MAXINDEX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.053 | 18.88M | 0.013 | 79.27M | 0.038 | 0.71× | 2.98× |
| 10,000 | 0.571 | 17.50M | 0.169 | 59.12M | 0.101 | 0.18× | 0.60× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
