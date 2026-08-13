# RollingAverageDrawdown benchmark (`AverageDrawdown` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.221 | 4.52M | 0.214 | 4.67M | 0.209 | 0.95× | 0.98× |
| 10,000 | 2.107 | 4.75M | 2.125 | 4.71M | 0.977 | 0.46× | 0.46× |
| 100,000 | 21.157 | 4.73M | 20.599 | 4.85M | 8.487 | 0.40× | 0.41× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.197 | 0.273 | 1.39× |
| 1 | 5 | 0.470 | 0.987 | 2.10× |
| 1 | 10 | 0.611 | 2.075 | 3.40× |
| 10 | 1 | 0.066 | 0.191 | 2.89× |
| 10 | 5 | 0.293 | 0.918 | 3.13× |
| 10 | 10 | 0.603 | 2.097 | 3.48× |
| 100 | 1 | 0.085 | 0.198 | 2.33× |
| 100 | 5 | 0.296 | 1.003 | 3.39× |
| 100 | 10 | 0.639 | 2.168 | 3.39× |
| 1,000 | 1 | 0.293 | 0.283 | 0.97× |
| 1,000 | 5 | 0.458 | 1.410 | 3.08× |
| 1,000 | 10 | 0.832 | 3.078 | 3.70× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
