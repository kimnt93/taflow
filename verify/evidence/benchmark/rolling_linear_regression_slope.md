# RollingLinearRegressionSlope benchmark (`LINEARREG_SLOPE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 71.23M | 0.013 | 75.63M | 0.041 | 2.92× | 3.10× |
| 10,000 | 0.108 | 92.35M | 0.108 | 92.58M | 0.140 | 1.29× | 1.30× |
| 100,000 | 1.100 | 90.95M | 1.052 | 95.05M | 1.075 | 0.98× | 1.02× |
| 1,000,000 | 11.657 | 85.79M | 10.449 | 95.70M | 10.389 | 0.89× | 0.99× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.106 | 0.164 | 1.55× |
| 1 | 5 | 0.458 | 0.605 | 1.32× |
| 1 | 10 | 0.553 | 0.943 | 1.71× |
| 10 | 1 | 0.059 | 0.100 | 1.69× |
| 10 | 5 | 0.265 | 0.499 | 1.88× |
| 10 | 10 | 0.475 | 0.930 | 1.96× |
| 100 | 1 | 0.052 | 0.090 | 1.74× |
| 100 | 5 | 0.225 | 0.461 | 2.04× |
| 100 | 10 | 0.532 | 0.938 | 1.77× |
| 1,000 | 1 | 0.059 | 0.102 | 1.73× |
| 1,000 | 5 | 0.225 | 0.478 | 2.13× |
| 1,000 | 10 | 0.514 | 1.096 | 2.13× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
