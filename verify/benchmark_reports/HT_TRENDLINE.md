# HilbertTransformTrendline benchmark (`HT_TRENDLINE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.127 | 7.86M | 0.078 | 12.81M | 0.082 | 0.64× | 1.05× |
| 10,000 | 1.185 | 8.44M | 0.797 | 12.54M | 0.623 | 0.53× | 0.78× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
