# RelativeStrengthIndex benchmark (`RSI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.049 | 20.55M | 0.008 | 130.53M | 0.037 | 0.76× | 4.80× |
| 10,000 | 0.469 | 21.30M | 0.067 | 148.46M | 0.088 | 0.19× | 1.31× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
