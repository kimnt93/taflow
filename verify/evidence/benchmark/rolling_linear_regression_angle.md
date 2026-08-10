# RollingLinearRegressionAngle benchmark (`LINEARREG_ANGLE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.046 | 21.86M | 0.046 | 21.67M | 0.058 | 1.27× | 1.26× |
| 10,000 | 0.246 | 40.73M | 0.276 | 36.21M | 0.307 | 1.25× | 1.11× |
| 100,000 | 2.439 | 41.00M | 2.620 | 38.16M | 2.475 | 1.01× | 0.94× |
| 1,000,000 | 24.039 | 41.60M | 23.070 | 43.35M | 22.647 | 0.94× | 0.98× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.086 | 0.122 | 1.43× |
| 1 | 5 | 0.351 | 0.539 | 1.54× |
| 1 | 10 | 0.472 | 0.903 | 1.91× |
| 10 | 1 | 0.050 | 0.094 | 1.89× |
| 10 | 5 | 0.237 | 0.464 | 1.96× |
| 10 | 10 | 0.517 | 0.967 | 1.87× |
| 100 | 1 | 0.057 | 0.100 | 1.74× |
| 100 | 5 | 0.246 | 0.474 | 1.93× |
| 100 | 10 | 0.523 | 0.986 | 1.88× |
| 1,000 | 1 | 0.070 | 0.112 | 1.60× |
| 1,000 | 5 | 0.255 | 0.601 | 2.36× |
| 1,000 | 10 | 0.610 | 1.589 | 2.61× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
