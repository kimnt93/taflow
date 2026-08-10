# FlagPennant benchmark (`FlagPennant` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 81.21M | 0.010 | 101.32M | 0.219 | 17.81× | 22.22× |
| 10,000 | 0.080 | 124.59M | 0.077 | 130.27M | 1.361 | 16.96× | 17.73× |
| 100,000 | 0.790 | 126.51M | 0.737 | 135.70M | 11.941 | 15.11× | 16.20× |
| 1,000,000 | 8.240 | 121.36M | 7.425 | 134.68M | 125.378 | 15.22× | 16.89× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.112 | 0.203 | 1.81× |
| 1 | 5 | 0.296 | 1.159 | 3.92× |
| 1 | 10 | 0.659 | 2.184 | 3.31× |
| 10 | 1 | 0.085 | 0.221 | 2.59× |
| 10 | 5 | 0.310 | 1.266 | 4.08× |
| 10 | 10 | 0.645 | 1.879 | 2.92× |
| 100 | 1 | 0.061 | 0.192 | 3.15× |
| 100 | 5 | 0.294 | 1.314 | 4.47× |
| 100 | 10 | 0.602 | 1.974 | 3.28× |
| 1,000 | 1 | 0.071 | 0.310 | 4.36× |
| 1,000 | 5 | 0.308 | 1.846 | 5.99× |
| 1,000 | 10 | 0.643 | 3.295 | 5.12× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
