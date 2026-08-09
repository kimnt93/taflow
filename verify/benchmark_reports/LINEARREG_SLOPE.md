# RollingLinearRegressionSlope benchmark (`LINEARREG_SLOPE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 69.27M | 0.013 | 76.41M | 0.041 | 2.84× | 3.13× |
| 10,000 | 0.111 | 90.24M | 0.110 | 91.05M | 0.134 | 1.21× | 1.22× |
| 100,000 | 1.147 | 87.21M | 1.093 | 91.50M | 1.088 | 0.95× | 1.00× |
| 1,000,000 | 11.255 | 88.85M | 10.797 | 92.62M | 10.435 | 0.93× | 0.97× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.074 | 0.139 | 1.87× |
| 1 | 5 | 0.322 | 0.533 | 1.65× |
| 1 | 10 | 0.544 | 0.993 | 1.83× |
| 10 | 1 | 0.050 | 0.093 | 1.85× |
| 10 | 5 | 0.218 | 0.443 | 2.03× |
| 10 | 10 | 0.526 | 0.995 | 1.89× |
| 100 | 1 | 0.059 | 0.112 | 1.91× |
| 100 | 5 | 0.238 | 0.475 | 2.00× |
| 100 | 10 | 0.488 | 1.001 | 2.05× |
| 1,000 | 1 | 0.066 | 0.109 | 1.67× |
| 1,000 | 5 | 0.280 | 0.582 | 2.08× |
| 1,000 | 10 | 0.531 | 1.146 | 2.16× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
