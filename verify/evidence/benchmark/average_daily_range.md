# AverageDailyRange benchmark (`AverageDailyRange` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.075 | 13.35M | 0.068 | 14.71M | 0.386 | 5.16× | 5.68× |
| 10,000 | 0.485 | 20.64M | 0.479 | 20.87M | 2.267 | 4.68× | 4.73× |
| 100,000 | 4.672 | 21.41M | 4.524 | 22.11M | 21.532 | 4.61× | 4.76× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.138 | 0.323 | 2.33× |
| 1 | 5 | 0.590 | 1.287 | 2.18× |
| 1 | 10 | 0.760 | 2.809 | 3.70× |
| 10 | 1 | 0.113 | 0.298 | 2.63× |
| 10 | 5 | 0.375 | 1.467 | 3.91× |
| 10 | 10 | 0.724 | 2.785 | 3.84× |
| 100 | 1 | 0.088 | 0.276 | 3.14× |
| 100 | 5 | 0.361 | 1.513 | 4.19× |
| 100 | 10 | 0.754 | 2.821 | 3.74× |
| 1,000 | 1 | 0.135 | 0.478 | 3.55× |
| 1,000 | 5 | 0.351 | 2.538 | 7.23× |
| 1,000 | 10 | 0.767 | 5.087 | 6.63× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
