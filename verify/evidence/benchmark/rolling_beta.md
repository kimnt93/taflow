# RollingBeta benchmark (`BETA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 52.32M | 0.018 | 54.22M | 0.080 | 4.18× | 4.34× |
| 10,000 | 0.126 | 79.09M | 0.108 | 92.27M | 0.178 | 1.41× | 1.64× |
| 100,000 | 1.216 | 82.25M | 1.200 | 83.34M | 1.313 | 1.08× | 1.09× |
| 1,000,000 | 17.532 | 57.04M | 10.586 | 94.47M | 9.784 | 0.56× | 0.92× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.096 | 0.146 | 1.53× |
| 1 | 5 | 0.959 | 1.431 | 1.49× |
| 1 | 10 | 2.510 | 2.118 | 0.84× |
| 10 | 1 | 0.059 | 0.113 | 1.93× |
| 10 | 5 | 0.297 | 0.702 | 2.37× |
| 10 | 10 | 1.310 | 1.418 | 1.08× |
| 100 | 1 | 0.073 | 0.123 | 1.69× |
| 100 | 5 | 0.381 | 0.664 | 1.74× |
| 100 | 10 | 0.937 | 2.615 | 2.79× |
| 1,000 | 1 | 0.101 | 0.216 | 2.15× |
| 1,000 | 5 | 0.353 | 0.684 | 1.94× |
| 1,000 | 10 | 0.621 | 1.531 | 2.46× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
