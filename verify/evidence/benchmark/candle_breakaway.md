# CandleBreakaway benchmark (`CDLBREAKAWAY` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 68.09M | 0.010 | 95.82M | 0.034 | 2.31× | 3.25× |
| 10,000 | 0.092 | 108.80M | 0.090 | 111.68M | 0.098 | 1.07× | 1.10× |
| 100,000 | 1.009 | 99.09M | 0.975 | 102.60M | 0.759 | 0.75× | 0.78× |
| 1,000,000 | 10.282 | 97.25M | 10.039 | 99.62M | 7.626 | 0.74× | 0.76× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.083 | 0.109 | 1.32× |
| 1 | 5 | 0.344 | 0.511 | 1.48× |
| 1 | 10 | 0.624 | 0.973 | 1.56× |
| 10 | 1 | 0.055 | 0.092 | 1.67× |
| 10 | 5 | 0.263 | 0.509 | 1.93× |
| 10 | 10 | 0.588 | 1.042 | 1.77× |
| 100 | 1 | 0.059 | 0.099 | 1.68× |
| 100 | 5 | 0.268 | 0.463 | 1.73× |
| 100 | 10 | 0.561 | 1.081 | 1.93× |
| 1,000 | 1 | 0.079 | 0.101 | 1.27× |
| 1,000 | 5 | 0.317 | 0.573 | 1.81× |
| 1,000 | 10 | 0.580 | 1.109 | 1.91× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
