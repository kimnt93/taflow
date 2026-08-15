# RollingAverageDrawdown benchmark (`AverageDrawdown` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.046 | 21.93M | 0.046 | 21.97M | 0.219 | 4.81× | 4.81× |
| 10,000 | 0.483 | 20.69M | 0.465 | 21.49M | 1.044 | 2.16× | 2.24× |
| 100,000 | 4.815 | 20.77M | 4.824 | 20.73M | 9.099 | 1.89× | 1.89× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.071 | 0.240 | 3.39× |
| 1 | 5 | 0.260 | 0.959 | 3.69× |
| 1 | 10 | 0.391 | 2.105 | 5.39× |
| 10 | 1 | 0.054 | 0.197 | 3.61× |
| 10 | 5 | 0.191 | 0.948 | 4.96× |
| 10 | 10 | 0.377 | 2.094 | 5.55× |
| 100 | 1 | 0.048 | 0.203 | 4.26× |
| 100 | 5 | 0.203 | 0.980 | 4.83× |
| 100 | 10 | 0.435 | 2.205 | 5.07× |
| 1,000 | 1 | 0.104 | 0.301 | 2.90× |
| 1,000 | 5 | 0.219 | 1.391 | 6.37× |
| 1,000 | 10 | 0.449 | 3.033 | 6.75× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
