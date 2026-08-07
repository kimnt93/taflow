# AverageDailyDollarValue benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 123.10M | 0.006 | 155.00M | nan | — | — |
| 10,000 | 0.054 | 185.54M | 0.051 | 196.84M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
