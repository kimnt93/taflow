# RollingLinearRegression benchmark (`LINEARREG` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 66.31M | 0.014 | 71.77M | 0.044 | 2.93× | 3.17× |
| 10,000 | 0.129 | 77.82M | 0.126 | 79.22M | 0.160 | 1.25× | 1.27× |
| 100,000 | 1.252 | 79.87M | 1.261 | 79.30M | 1.376 | 1.10× | 1.09× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.070 | 0.128 | 1.84× |
| 1 | 5 | 0.206 | 0.466 | 2.27× |
| 1 | 10 | 0.403 | 0.955 | 2.37× |
| 10 | 1 | 0.048 | 0.087 | 1.83× |
| 10 | 5 | 0.193 | 0.467 | 2.41× |
| 10 | 10 | 0.402 | 0.934 | 2.32× |
| 100 | 1 | 0.043 | 0.094 | 2.17× |
| 100 | 5 | 0.186 | 0.432 | 2.32× |
| 100 | 10 | 0.428 | 0.968 | 2.26× |
| 1,000 | 1 | 0.055 | 0.105 | 1.92× |
| 1,000 | 5 | 0.199 | 0.508 | 2.56× |
| 1,000 | 10 | 0.414 | 1.113 | 2.69× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
