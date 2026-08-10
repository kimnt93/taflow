# MathDivide benchmark (`DIV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 203.49M | 0.004 | 272.63M | 0.032 | 6.44× | 8.63× |
| 10,000 | 0.012 | 803.41M | 0.009 | 1.08G | 0.037 | 2.95× | 3.96× |
| 100,000 | 0.084 | 1.19G | 0.057 | 1.77G | 0.082 | 0.98× | 1.45× |
| 1,000,000 | 1.262 | 792.61M | 0.953 | 1.05G | 1.084 | 0.86× | 1.14× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.189 | 0.194 | 1.03× |
| 1 | 5 | 0.246 | 0.443 | 1.80× |
| 1 | 10 | 0.475 | 0.910 | 1.91× |
| 10 | 1 | 0.061 | 0.096 | 1.59× |
| 10 | 5 | 0.245 | 0.480 | 1.96× |
| 10 | 10 | 0.490 | 0.886 | 1.81× |
| 100 | 1 | 0.050 | 0.091 | 1.81× |
| 100 | 5 | 0.245 | 0.428 | 1.75× |
| 100 | 10 | 0.483 | 0.900 | 1.87× |
| 1,000 | 1 | 0.049 | 0.090 | 1.85× |
| 1,000 | 5 | 0.245 | 0.424 | 1.73× |
| 1,000 | 10 | 0.460 | 0.961 | 2.09× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
