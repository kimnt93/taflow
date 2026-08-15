# BalanceOfPower benchmark (`BOP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 316.18M | 0.001 | 669.25M | 0.029 | 9.26× | 19.60× |
| 10,000 | 0.010 | 955.94M | 0.007 | 1.43G | 0.039 | 3.70× | 5.54× |
| 100,000 | 0.086 | 1.16G | 0.061 | 1.63G | 0.131 | 1.52× | 2.13× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.130 | 0.102 | 0.78× |
| 1 | 5 | 0.238 | 0.431 | 1.81× |
| 1 | 10 | 0.398 | 0.899 | 2.26× |
| 10 | 1 | 0.043 | 0.085 | 1.96× |
| 10 | 5 | 0.193 | 0.419 | 2.17× |
| 10 | 10 | 0.386 | 0.931 | 2.41× |
| 100 | 1 | 0.049 | 0.101 | 2.06× |
| 100 | 5 | 0.192 | 0.424 | 2.21× |
| 100 | 10 | 0.386 | 0.878 | 2.27× |
| 1,000 | 1 | 0.041 | 0.086 | 2.12× |
| 1,000 | 5 | 0.179 | 0.458 | 2.56× |
| 1,000 | 10 | 0.406 | 0.932 | 2.30× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
