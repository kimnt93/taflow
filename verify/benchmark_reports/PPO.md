# PercentagePriceOscillator benchmark (`PPO` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.056 | 18.00M | 0.012 | 80.33M | 0.040 | 0.72× | 3.23× |
| 10,000 | 0.528 | 18.96M | 0.117 | 85.54M | 0.084 | 0.16× | 0.72× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
