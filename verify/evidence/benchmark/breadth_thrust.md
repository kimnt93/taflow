# BreadthThrust benchmark (`BreadthThrust` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 102.02M | 0.008 | 121.79M | 8.227 | 839.24× | 1001.89× |
| 10,000 | 0.057 | 175.57M | 0.055 | 182.40M | 80.360 | 1410.86× | 1465.75× |
| 100,000 | 0.531 | 188.22M | 0.613 | 163.05M | 804.115 | 1513.48× | 1311.14× |
| 1,000,000 | 7.329 | 136.44M | 6.054 | 165.18M | 8035.995 | 1096.46× | 1327.38× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.149 | 0.305 | 2.05× |
| 1 | 5 | 0.253 | 1.491 | 5.88× |
| 1 | 10 | 0.500 | 2.723 | 5.44× |
| 10 | 1 | 0.056 | 0.323 | 5.76× |
| 10 | 5 | 0.241 | 1.599 | 6.64× |
| 10 | 10 | 0.519 | 3.608 | 6.95× |
| 100 | 1 | 0.055 | 1.089 | 19.64× |
| 100 | 5 | 0.253 | 5.589 | 22.12× |
| 100 | 10 | 0.530 | 12.939 | 24.43× |
| 1,000 | 1 | 0.062 | 8.691 | 140.09× |
| 1,000 | 5 | 0.310 | 44.541 | 143.52× |
| 1,000 | 10 | 0.558 | 98.395 | 176.39× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
