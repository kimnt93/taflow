# RollingQuantile benchmark (`RollingQuantile` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.324 | 3.09M | 0.359 | 2.79M | 0.334 | 1.03× | 0.93× |
| 10,000 | 3.016 | 3.32M | 3.052 | 3.28M | 1.743 | 0.58× | 0.57× |
| 100,000 | 30.539 | 3.27M | 30.242 | 3.31M | 16.580 | 0.54× | 0.55× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.119 | 0.302 | 2.53× |
| 1 | 5 | 0.340 | 1.393 | 4.10× |
| 1 | 10 | 0.618 | 2.519 | 4.08× |
| 10 | 1 | 0.073 | 0.244 | 3.34× |
| 10 | 5 | 0.302 | 1.381 | 4.58× |
| 10 | 10 | 0.633 | 2.660 | 4.20× |
| 100 | 1 | 0.098 | 0.256 | 2.62× |
| 100 | 5 | 0.325 | 1.481 | 4.56× |
| 100 | 10 | 0.673 | 2.691 | 4.00× |
| 1,000 | 1 | 0.388 | 0.408 | 1.05× |
| 1,000 | 5 | 0.630 | 2.245 | 3.56× |
| 1,000 | 10 | 0.967 | 4.467 | 4.62× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
