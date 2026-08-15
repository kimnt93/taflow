# RollingLinearRegressionAngle benchmark (`LINEARREG_ANGLE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.024 | 42.08M | 0.022 | 44.60M | 0.053 | 2.22× | 2.36× |
| 10,000 | 0.229 | 43.64M | 0.219 | 45.61M | 0.246 | 1.07× | 1.12× |
| 100,000 | 2.255 | 44.35M | 2.142 | 46.69M | 2.244 | 1.00× | 1.05× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.123 | 0.146 | 1.19× |
| 1 | 5 | 0.247 | 0.493 | 2.00× |
| 1 | 10 | 0.413 | 0.949 | 2.30× |
| 10 | 1 | 0.042 | 0.091 | 2.18× |
| 10 | 5 | 0.184 | 0.415 | 2.26× |
| 10 | 10 | 0.382 | 0.966 | 2.53× |
| 100 | 1 | 0.049 | 0.087 | 1.76× |
| 100 | 5 | 0.208 | 0.436 | 2.10× |
| 100 | 10 | 0.423 | 0.957 | 2.26× |
| 1,000 | 1 | 0.070 | 0.121 | 1.72× |
| 1,000 | 5 | 0.210 | 0.553 | 2.64× |
| 1,000 | 10 | 0.823 | 1.130 | 1.37× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
