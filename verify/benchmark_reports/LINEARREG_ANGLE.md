# RollingLinearRegressionAngle benchmark (`LINEARREG_ANGLE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.027 | 36.59M | 0.025 | 39.49M | 0.063 | 2.30× | 2.48× |
| 10,000 | 0.262 | 38.12M | 0.229 | 43.76M | 0.267 | 1.02× | 1.17× |
| 100,000 | 2.517 | 39.73M | 2.457 | 40.70M | 2.315 | 0.92× | 0.94× |
| 1,000,000 | 24.644 | 40.58M | 23.823 | 41.98M | 22.930 | 0.93× | 0.96× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.074 | 0.123 | 1.66× |
| 1 | 5 | 0.246 | 0.466 | 1.89× |
| 1 | 10 | 0.538 | 1.002 | 1.86× |
| 10 | 1 | 0.057 | 0.096 | 1.68× |
| 10 | 5 | 0.247 | 0.464 | 1.88× |
| 10 | 10 | 0.505 | 0.972 | 1.93× |
| 100 | 1 | 0.053 | 0.092 | 1.73× |
| 100 | 5 | 0.230 | 0.468 | 2.04× |
| 100 | 10 | 0.504 | 1.040 | 2.06× |
| 1,000 | 1 | 0.078 | 0.129 | 1.66× |
| 1,000 | 5 | 0.249 | 0.624 | 2.50× |
| 1,000 | 10 | 0.554 | 1.172 | 2.12× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
