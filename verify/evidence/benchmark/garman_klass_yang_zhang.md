# GarmanKlassYangZhang benchmark (`annualized Garman-Klass-Yang-Zhang volatility` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.027 | 36.53M | 0.024 | 41.48M | 0.136 | 4.98× | 5.65× |
| 10,000 | 0.275 | 36.38M | 0.219 | 45.61M | 0.476 | 1.73× | 2.17× |
| 100,000 | 2.273 | 44.00M | 2.293 | 43.60M | 4.110 | 1.81× | 1.79× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.078 | 0.168 | 2.15× |
| 1 | 5 | 0.425 | 0.691 | 1.63× |
| 1 | 10 | 0.432 | 1.315 | 3.04× |
| 10 | 1 | 0.049 | 0.131 | 2.65× |
| 10 | 5 | 0.248 | 0.659 | 2.66× |
| 10 | 10 | 0.459 | 1.272 | 2.77× |
| 100 | 1 | 0.052 | 0.155 | 2.97× |
| 100 | 5 | 0.217 | 0.923 | 4.25× |
| 100 | 10 | 0.468 | 1.693 | 3.61× |
| 1,000 | 1 | 0.072 | 0.195 | 2.72× |
| 1,000 | 5 | 0.223 | 1.318 | 5.91× |
| 1,000 | 10 | 0.456 | 2.483 | 5.44× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
