# RollingOmegaRatio benchmark (`OmegaRatio` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.031 | 32.63M | 0.030 | 32.95M | 0.200 | 6.51× | 6.58× |
| 10,000 | 0.297 | 33.62M | 0.281 | 35.63M | 0.663 | 2.23× | 2.36× |
| 100,000 | 2.816 | 35.51M | 2.747 | 36.41M | 5.953 | 2.11× | 2.17× |
| 1,000,000 | 28.593 | 34.97M | 27.742 | 36.05M | 53.163 | 1.86× | 1.92× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.107 | 0.276 | 2.57× |
| 1 | 5 | 0.345 | 1.329 | 3.86× |
| 1 | 10 | 0.478 | 2.505 | 5.24× |
| 10 | 1 | 0.058 | 0.233 | 4.02× |
| 10 | 5 | 0.219 | 1.301 | 5.93× |
| 10 | 10 | 0.460 | 2.472 | 5.37× |
| 100 | 1 | 0.064 | 0.235 | 3.66× |
| 100 | 5 | 0.232 | 1.338 | 5.76× |
| 100 | 10 | 0.491 | 2.471 | 5.03× |
| 1,000 | 1 | 0.077 | 0.285 | 3.71× |
| 1,000 | 5 | 0.253 | 1.604 | 6.33× |
| 1,000 | 10 | 0.511 | 3.010 | 5.89× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
