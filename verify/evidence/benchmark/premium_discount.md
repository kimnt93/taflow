# PremiumDiscount benchmark (`rolling premium-discount zone` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.024 | 42.04M | 0.023 | 43.72M | 3.384 | 142.24× | 147.95× |
| 10,000 | 0.268 | 37.28M | 0.268 | 37.31M | 32.347 | 120.60× | 120.68× |
| 100,000 | 2.779 | 35.98M | 2.733 | 36.59M | 324.921 | 116.91× | 118.90× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.173 | 0.128 | 0.74× |
| 1 | 5 | 0.270 | 0.487 | 1.80× |
| 1 | 10 | 0.473 | 0.985 | 2.08× |
| 10 | 1 | 0.049 | 0.127 | 2.58× |
| 10 | 5 | 0.238 | 0.623 | 2.62× |
| 10 | 10 | 0.514 | 1.327 | 2.58× |
| 100 | 1 | 0.053 | 0.413 | 7.81× |
| 100 | 5 | 0.231 | 2.240 | 9.70× |
| 100 | 10 | 0.535 | 4.187 | 7.83× |
| 1,000 | 1 | 0.085 | 3.358 | 39.60× |
| 1,000 | 5 | 0.269 | 17.602 | 65.55× |
| 1,000 | 10 | 0.621 | 35.894 | 57.85× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
