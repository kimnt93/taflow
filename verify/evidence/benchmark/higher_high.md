# HigherHigh benchmark (`higher high relation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.002 | 417.07M | 0.001 | 931.96M | 0.015 | 6.39× | 14.29× |
| 10,000 | 0.007 | 1.45G | 0.004 | 2.52G | 0.024 | 3.54× | 6.16× |
| 100,000 | 0.057 | 1.76G | 0.036 | 2.76G | 0.102 | 1.80× | 2.82× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.055 | 0.074 | 1.35× |
| 1 | 5 | 0.254 | 0.315 | 1.24× |
| 1 | 10 | 0.352 | 0.667 | 1.89× |
| 10 | 1 | 0.038 | 0.067 | 1.76× |
| 10 | 5 | 0.181 | 0.307 | 1.70× |
| 10 | 10 | 0.392 | 0.687 | 1.75× |
| 100 | 1 | 0.044 | 0.063 | 1.44× |
| 100 | 5 | 0.178 | 0.343 | 1.93× |
| 100 | 10 | 0.386 | 0.682 | 1.76× |
| 1,000 | 1 | 0.053 | 0.070 | 1.34× |
| 1,000 | 5 | 0.191 | 0.340 | 1.78× |
| 1,000 | 10 | 0.374 | 0.759 | 2.03× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
