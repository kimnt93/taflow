# Liquidity benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.188 | 5.31M | 0.189 | 5.30M | nan | — | — |
| 10,000 | 2.274 | 4.40M | 2.279 | 4.39M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
