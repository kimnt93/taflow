# RollingLinearRegressionAngle benchmark (`LINEARREG_ANGLE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.184 | 5.44M | 0.181 | 5.51M | 0.048 | 0.26× | 0.27× |
| 10,000 | 1.776 | 5.63M | 1.837 | 5.44M | 0.241 | 0.14× | 0.13× |
| 100,000 | 18.387 | 5.44M | 18.574 | 5.38M | 2.253 | 0.12× | 0.12× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.113 | 0.108 | 0.96× |
| 1 | 5 | 0.365 | 0.525 | 1.44× |
| 1 | 10 | 0.774 | 1.179 | 1.52× |
| 10 | 1 | 0.084 | 0.095 | 1.13× |
| 10 | 5 | 0.316 | 0.498 | 1.58× |
| 10 | 10 | 0.669 | 0.984 | 1.47× |
| 100 | 1 | 0.090 | 0.096 | 1.07× |
| 100 | 5 | 0.330 | 0.472 | 1.43× |
| 100 | 10 | 0.692 | 0.994 | 1.44× |
| 1,000 | 1 | 0.259 | 0.118 | 0.45× |
| 1,000 | 5 | 0.530 | 0.581 | 1.10× |
| 1,000 | 10 | 0.820 | 1.262 | 1.54× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
