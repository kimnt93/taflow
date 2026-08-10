# RollingLinearRegressionAngle benchmark (`LINEARREG_ANGLE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.025 | 40.67M | 0.023 | 43.87M | 0.049 | 2.00× | 2.15× |
| 10,000 | 0.228 | 43.82M | 0.207 | 48.31M | 0.232 | 1.01× | 1.12× |
| 100,000 | 2.098 | 47.67M | 2.092 | 47.79M | 2.168 | 1.03× | 1.04× |
| 1,000,000 | 21.692 | 46.10M | 21.701 | 46.08M | 20.615 | 0.95× | 0.95× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.098 | 0.105 | 1.07× |
| 1 | 5 | 0.296 | 0.469 | 1.59× |
| 1 | 10 | 0.451 | 0.891 | 1.97× |
| 10 | 1 | 0.049 | 0.090 | 1.84× |
| 10 | 5 | 0.223 | 0.453 | 2.03× |
| 10 | 10 | 0.481 | 0.994 | 2.07× |
| 100 | 1 | 0.054 | 0.094 | 1.75× |
| 100 | 5 | 0.238 | 0.462 | 1.94× |
| 100 | 10 | 0.514 | 0.920 | 1.79× |
| 1,000 | 1 | 0.074 | 0.113 | 1.52× |
| 1,000 | 5 | 0.278 | 0.597 | 2.15× |
| 1,000 | 10 | 0.524 | 1.119 | 2.13× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
