# RollingMaximumDrawdown benchmark (`MaxDrawdown` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.042 | 23.95M | 0.041 | 24.59M | 0.244 | 5.85× | 6.01× |
| 10,000 | 0.404 | 24.76M | 0.396 | 25.28M | 1.219 | 3.02× | 3.08× |
| 100,000 | 4.104 | 24.36M | 3.974 | 25.16M | 11.584 | 2.82× | 2.92× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.077 | 0.222 | 2.89× |
| 1 | 5 | 0.289 | 1.250 | 4.33× |
| 1 | 10 | 0.435 | 2.084 | 4.79× |
| 10 | 1 | 0.046 | 0.193 | 4.24× |
| 10 | 5 | 0.183 | 0.922 | 5.03× |
| 10 | 10 | 0.427 | 2.107 | 4.93× |
| 100 | 1 | 0.073 | 0.197 | 2.69× |
| 100 | 5 | 0.192 | 1.031 | 5.36× |
| 100 | 10 | 0.454 | 2.208 | 4.87× |
| 1,000 | 1 | 0.087 | 0.312 | 3.57× |
| 1,000 | 5 | 0.208 | 1.606 | 7.71× |
| 1,000 | 10 | 0.470 | 3.421 | 7.27× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
