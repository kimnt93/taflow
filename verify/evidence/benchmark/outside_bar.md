# OutsideBar benchmark (`outside bar relation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 385.08M | 0.001 | 705.98M | 0.022 | 8.37× | 15.35× |
| 10,000 | 0.010 | 1.01G | 0.007 | 1.48G | 0.040 | 4.07× | 5.94× |
| 100,000 | 0.098 | 1.02G | 0.076 | 1.32G | 0.237 | 2.42× | 3.14× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.083 | 0.080 | 0.97× |
| 1 | 5 | 0.235 | 0.345 | 1.47× |
| 1 | 10 | 0.389 | 0.734 | 1.89× |
| 10 | 1 | 0.048 | 0.069 | 1.42× |
| 10 | 5 | 0.168 | 0.350 | 2.08× |
| 10 | 10 | 0.402 | 0.760 | 1.89× |
| 100 | 1 | 0.042 | 0.069 | 1.62× |
| 100 | 5 | 0.183 | 0.346 | 1.89× |
| 100 | 10 | 0.377 | 0.708 | 1.88× |
| 1,000 | 1 | 0.038 | 0.078 | 2.04× |
| 1,000 | 5 | 0.199 | 0.490 | 2.47× |
| 1,000 | 10 | 0.373 | 1.240 | 3.33× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
