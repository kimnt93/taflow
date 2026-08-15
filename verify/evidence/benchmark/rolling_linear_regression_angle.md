# RollingLinearRegressionAngle benchmark (`LINEARREG_ANGLE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.023 | 43.17M | 0.021 | 46.60M | 0.051 | 2.18× | 2.36× |
| 10,000 | 0.227 | 44.02M | 0.213 | 47.05M | 0.259 | 1.14× | 1.22× |
| 100,000 | 2.148 | 46.55M | 2.168 | 46.14M | 2.217 | 1.03× | 1.02× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.151 | 0.132 | 0.87× |
| 1 | 5 | 0.215 | 0.453 | 2.10× |
| 1 | 10 | 0.374 | 0.947 | 2.53× |
| 10 | 1 | 0.045 | 0.100 | 2.23× |
| 10 | 5 | 0.222 | 0.461 | 2.08× |
| 10 | 10 | 0.379 | 0.914 | 2.41× |
| 100 | 1 | 0.045 | 0.092 | 2.04× |
| 100 | 5 | 0.210 | 0.461 | 2.20× |
| 100 | 10 | 0.454 | 0.936 | 2.06× |
| 1,000 | 1 | 0.063 | 0.113 | 1.78× |
| 1,000 | 5 | 0.202 | 0.547 | 2.71× |
| 1,000 | 10 | 0.436 | 1.276 | 2.93× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
