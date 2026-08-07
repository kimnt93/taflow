# HilbertTransformTrendMode benchmark (`HT_TRENDMODE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.484 | 2.07M | 0.484 | 2.07M | 0.486 | 1.00× | 1.00× |
| 10,000 | 5.133 | 1.95M | 5.010 | 2.00M | 4.919 | 0.96× | 0.98× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
