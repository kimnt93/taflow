# SessionVolumeLevels benchmark (`anchored volume levels` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.053 | 18.72M | 0.048 | 20.67M | 14.134 | 264.56× | 292.20× |
| 10,000 | 0.501 | 19.96M | 0.497 | 20.12M | 147.779 | 295.03× | 297.29× |
| 100,000 | 5.396 | 18.53M | 4.825 | 20.73M | 1450.302 | 268.78× | 300.59× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.159 | 0.174 | 1.09× |
| 1 | 5 | 0.265 | 0.725 | 2.73× |
| 1 | 10 | 0.425 | 1.327 | 3.12× |
| 10 | 1 | 0.048 | 0.296 | 6.18× |
| 10 | 5 | 0.228 | 1.605 | 7.05× |
| 10 | 10 | 0.469 | 3.303 | 7.04× |
| 100 | 1 | 0.055 | 1.869 | 33.83× |
| 100 | 5 | 0.221 | 10.058 | 45.48× |
| 100 | 10 | 0.489 | 20.477 | 41.90× |
| 1,000 | 1 | 0.102 | 14.958 | 147.11× |
| 1,000 | 5 | 0.733 | 79.465 | 108.38× |
| 1,000 | 10 | 1.017 | 163.065 | 160.31× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
