# CandleBreakaway benchmark (`CDLBREAKAWAY` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.027 | 36.85M | 0.025 | 40.52M | 0.031 | 1.15× | 1.27× |
| 10,000 | 0.310 | 32.22M | 0.307 | 32.57M | 0.088 | 0.28× | 0.29× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
