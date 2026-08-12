# RollingBetaNeutralSpread benchmark (`BetaNeutralSpread` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.053 | 18.71M | 0.049 | 20.25M | 0.239 | 4.47× | 4.83× |
| 10,000 | 0.521 | 19.20M | 0.601 | 16.64M | 0.944 | 1.81× | 1.57× |
| 100,000 | 4.822 | 20.74M | 4.946 | 20.22M | 8.773 | 1.82× | 1.77× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.069 | 0.363 | 5.25× |
| 1 | 5 | 0.354 | 1.242 | 3.51× |
| 1 | 10 | 0.533 | 9.685 | 18.18× |
| 10 | 1 | 0.061 | 0.223 | 3.64× |
| 10 | 5 | 0.235 | 1.310 | 5.57× |
| 10 | 10 | 0.543 | 2.505 | 4.62× |
| 100 | 1 | 0.066 | 0.242 | 3.69× |
| 100 | 5 | 0.302 | 1.232 | 4.07× |
| 100 | 10 | 0.525 | 2.490 | 4.74× |
| 1,000 | 1 | 0.118 | 0.307 | 2.61× |
| 1,000 | 5 | 0.280 | 1.745 | 6.22× |
| 1,000 | 10 | 0.573 | 3.559 | 6.21× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
