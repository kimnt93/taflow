# RollingLinearRegression benchmark (`LINEARREG` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 57.02M | 0.016 | 60.91M | 0.045 | 2.56× | 2.74× |
| 10,000 | 0.135 | 74.00M | 0.136 | 73.64M | 0.175 | 1.29× | 1.29× |
| 100,000 | 1.313 | 76.16M | 1.291 | 77.46M | 1.467 | 1.12× | 1.14× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.090 | 0.168 | 1.88× |
| 1 | 5 | 0.317 | 0.498 | 1.57× |
| 1 | 10 | 0.468 | 0.943 | 2.01× |
| 10 | 1 | 0.052 | 0.093 | 1.79× |
| 10 | 5 | 0.220 | 0.430 | 1.96× |
| 10 | 10 | 0.497 | 0.962 | 1.94× |
| 100 | 1 | 0.052 | 0.101 | 1.92× |
| 100 | 5 | 0.225 | 0.446 | 1.98× |
| 100 | 10 | 0.508 | 0.996 | 1.96× |
| 1,000 | 1 | 0.068 | 0.120 | 1.77× |
| 1,000 | 5 | 0.224 | 0.500 | 2.24× |
| 1,000 | 10 | 0.500 | 1.028 | 2.06× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
