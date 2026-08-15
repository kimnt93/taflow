# LowerLow benchmark (`lower low relation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 192.52M | 0.004 | 268.20M | 0.022 | 4.28× | 5.96× |
| 10,000 | 0.029 | 346.28M | 0.027 | 370.72M | 0.040 | 1.39× | 1.48× |
| 100,000 | 0.275 | 363.01M | 0.258 | 387.85M | 0.224 | 0.81× | 0.87× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.096 | 0.130 | 1.35× |
| 1 | 5 | 0.311 | 0.386 | 1.24× |
| 1 | 10 | 0.403 | 0.742 | 1.84× |
| 10 | 1 | 0.041 | 0.078 | 1.89× |
| 10 | 5 | 0.185 | 0.343 | 1.85× |
| 10 | 10 | 0.382 | 0.747 | 1.96× |
| 100 | 1 | 0.048 | 0.074 | 1.52× |
| 100 | 5 | 0.200 | 0.349 | 1.74× |
| 100 | 10 | 0.390 | 0.744 | 1.91× |
| 1,000 | 1 | 0.047 | 0.077 | 1.63× |
| 1,000 | 5 | 0.212 | 0.483 | 2.28× |
| 1,000 | 10 | 0.393 | 1.168 | 2.97× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
