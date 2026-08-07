# AccumulationDistributionOscillator benchmark (`ADOSC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.052 | 19.27M | 0.009 | 109.79M | 0.036 | 0.70× | 3.96× |
| 10,000 | 0.475 | 21.07M | 0.074 | 135.40M | 0.059 | 0.12× | 0.79× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
