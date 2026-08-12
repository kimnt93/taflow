# RollingLinearRegressionAngle benchmark (`LINEARREG_ANGLE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.024 | 40.84M | 0.023 | 43.51M | 0.050 | 2.05× | 2.18× |
| 10,000 | 0.229 | 43.67M | 0.226 | 44.21M | 0.246 | 1.08× | 1.09× |
| 100,000 | 2.204 | 45.38M | 2.193 | 45.61M | 2.415 | 1.10× | 1.10× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.089 | 0.120 | 1.36× |
| 1 | 5 | 0.266 | 0.486 | 1.82× |
| 1 | 10 | 0.526 | 0.927 | 1.76× |
| 10 | 1 | 0.056 | 0.092 | 1.65× |
| 10 | 5 | 0.259 | 0.477 | 1.84× |
| 10 | 10 | 0.502 | 0.948 | 1.89× |
| 100 | 1 | 0.053 | 0.088 | 1.65× |
| 100 | 5 | 0.251 | 0.478 | 1.91× |
| 100 | 10 | 0.537 | 1.001 | 1.86× |
| 1,000 | 1 | 0.077 | 0.119 | 1.54× |
| 1,000 | 5 | 0.269 | 0.595 | 2.21× |
| 1,000 | 10 | 0.540 | 1.190 | 2.20× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
