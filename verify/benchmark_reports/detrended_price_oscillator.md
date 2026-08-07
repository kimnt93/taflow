# DetrendedPriceOscillator benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 107.09M | 0.008 | 119.59M | nan | — | — |
| 10,000 | 0.075 | 132.48M | 0.072 | 138.57M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
