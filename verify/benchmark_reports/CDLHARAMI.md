# CandleHarami benchmark (`CDLHARAMI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.031 | 32.33M | 0.029 | 34.74M | 0.035 | 1.13× | 1.22× |
| 10,000 | 0.314 | 31.87M | 0.312 | 32.05M | 0.145 | 0.46× | 0.46× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
