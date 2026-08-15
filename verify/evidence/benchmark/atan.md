# MathAtan benchmark (`ATAN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 137.52M | 0.006 | 158.39M | 0.035 | 4.77× | 5.49× |
| 10,000 | 0.061 | 163.97M | 0.058 | 172.31M | 0.087 | 1.42× | 1.50× |
| 100,000 | 0.623 | 160.44M | 0.576 | 173.68M | 0.642 | 1.03× | 1.12× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.121 | 0.190 | 1.58× |
| 1 | 5 | 0.211 | 0.474 | 2.24× |
| 1 | 10 | 0.436 | 0.994 | 2.28× |
| 10 | 1 | 0.042 | 0.088 | 2.06× |
| 10 | 5 | 0.182 | 0.415 | 2.28× |
| 10 | 10 | 0.372 | 0.867 | 2.33× |
| 100 | 1 | 0.055 | 0.087 | 1.59× |
| 100 | 5 | 0.199 | 0.439 | 2.21× |
| 100 | 10 | 0.411 | 0.882 | 2.15× |
| 1,000 | 1 | 0.052 | 0.094 | 1.81× |
| 1,000 | 5 | 0.195 | 0.441 | 2.27× |
| 1,000 | 10 | 0.455 | 0.963 | 2.12× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
