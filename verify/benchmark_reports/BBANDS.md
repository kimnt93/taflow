# BollingerBands benchmark (`BBANDS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.211 | 4.74M | 0.025 | 40.17M | 0.053 | 0.25× | 2.15× |
| 10,000 | 2.246 | 4.45M | 0.227 | 44.13M | 0.105 | 0.05× | 0.46× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
