# MinusDirectionalIndicator benchmark (`MINUS_DI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 81.20M | 0.011 | 88.06M | 0.039 | 3.13× | 3.40× |
| 10,000 | 0.104 | 96.57M | 0.102 | 98.30M | 0.101 | 0.98× | 1.00× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
