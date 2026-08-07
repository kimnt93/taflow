# CandleHammer benchmark (`CDLHAMMER` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 62.98M | 0.014 | 72.69M | 0.042 | 2.66× | 3.07× |
| 10,000 | 0.122 | 82.11M | 0.121 | 82.74M | 0.177 | 1.45× | 1.46× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
