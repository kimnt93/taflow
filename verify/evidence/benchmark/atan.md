# MathAtan benchmark (`ATAN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 102.88M | 0.009 | 111.59M | 0.034 | 3.53× | 3.83× |
| 10,000 | 0.069 | 145.10M | 0.071 | 141.50M | 0.096 | 1.39× | 1.36× |
| 100,000 | 0.705 | 141.85M | 0.662 | 151.01M | 0.745 | 1.06× | 1.13× |
| 1,000,000 | 7.232 | 138.27M | 6.774 | 147.62M | 7.860 | 1.09× | 1.16× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.127 | 0.130 | 1.02× |
| 1 | 5 | 0.289 | 0.561 | 1.94× |
| 1 | 10 | 0.654 | 1.111 | 1.70× |
| 10 | 1 | 0.052 | 0.089 | 1.70× |
| 10 | 5 | 0.240 | 0.623 | 2.59× |
| 10 | 10 | 0.580 | 1.301 | 2.24× |
| 100 | 1 | 0.050 | 0.099 | 2.01× |
| 100 | 5 | 0.238 | 0.532 | 2.24× |
| 100 | 10 | 0.561 | 1.069 | 1.91× |
| 1,000 | 1 | 0.063 | 0.101 | 1.60× |
| 1,000 | 5 | 0.254 | 0.497 | 1.96× |
| 1,000 | 10 | 0.571 | 1.258 | 2.20× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
