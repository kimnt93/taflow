# MathFloor benchmark (`FLOOR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 169.68M | 0.005 | 209.58M | 0.029 | 4.93× | 6.09× |
| 10,000 | 0.028 | 357.60M | 0.030 | 335.13M | 0.049 | 1.75× | 1.64× |
| 100,000 | 0.279 | 358.72M | 0.272 | 367.97M | 0.196 | 0.70× | 0.72× |
| 1,000,000 | 3.835 | 260.73M | 3.009 | 332.33M | 2.732 | 0.71× | 0.91× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.099 | 0.136 | 1.37× |
| 1 | 5 | 1.350 | 0.613 | 0.45× |
| 1 | 10 | 2.492 | 1.218 | 0.49× |
| 10 | 1 | 0.052 | 0.081 | 1.57× |
| 10 | 5 | 0.366 | 0.648 | 1.77× |
| 10 | 10 | 2.712 | 1.504 | 0.55× |
| 100 | 1 | 0.054 | 0.111 | 2.06× |
| 100 | 5 | 0.291 | 0.470 | 1.61× |
| 100 | 10 | 0.510 | 0.967 | 1.90× |
| 1,000 | 1 | 0.059 | 0.096 | 1.62× |
| 1,000 | 5 | 0.253 | 0.465 | 1.84× |
| 1,000 | 10 | 0.564 | 1.006 | 1.78× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
