# RollingRank benchmark (`rolling percentile rank` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.485 | 2.06M | 0.481 | 2.08M | 0.132 | 0.27× | 0.27× |
| 10,000 | 4.734 | 2.11M | 4.755 | 2.10M | 0.725 | 0.15× | 0.15× |
| 100,000 | 47.143 | 2.12M | 46.980 | 2.13M | 6.564 | 0.14× | 0.14× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.234 | 0.133 | 0.57× |
| 1 | 5 | 0.405 | 0.690 | 1.70× |
| 1 | 10 | 0.560 | 1.075 | 1.92× |
| 10 | 1 | 0.066 | 0.107 | 1.62× |
| 10 | 5 | 0.289 | 0.504 | 1.75× |
| 10 | 10 | 0.590 | 1.077 | 1.83× |
| 100 | 1 | 0.109 | 0.167 | 1.53× |
| 100 | 5 | 0.306 | 0.771 | 2.52× |
| 100 | 10 | 0.663 | 1.619 | 2.44× |
| 1,000 | 1 | 0.558 | 0.220 | 0.39× |
| 1,000 | 5 | 0.860 | 0.928 | 1.08× |
| 1,000 | 10 | 1.355 | 2.020 | 1.49× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
